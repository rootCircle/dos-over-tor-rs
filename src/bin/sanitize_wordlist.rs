use std::fs::{remove_file, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

fn visit_and_write(
    value: &str,
    file_opts: &OpenOptions,
    destination_file: &str,
) -> Result<(), anyhow::Error> {
    let mut file = file_opts.open(destination_file)?;

    file.write_all(value.as_bytes())?;

    Ok(())
}
pub async fn run(source_file: &str, destination_file: &'static str) -> Result<(), anyhow::Error> {
    let file = File::open(source_file)?;

    // Buffer the read
    let reader = BufReader::new(file);

    for line in reader.lines() {
        tokio::spawn(async move {
            let parsed_line = line;
            if parsed_line.is_ok() {
                let parsed_line = &parsed_line.unwrap();
                if parsed_line.is_ascii()
                    && parsed_line.chars().all(char::is_uppercase)
                    && parsed_line.len() <= 7
                {
                    let mut file_opts = OpenOptions::new();
                    file_opts.create(true); // Create the file if it doesn't exist.
                    file_opts.append(true); // Append data to the file.

                    let mut parsed_line = parsed_line.to_ascii_uppercase();
                    parsed_line.push('\n');

                    visit_and_write(&parsed_line, &file_opts, destination_file).unwrap();
                }
            }
        });
    }

    tokio::join!();
    Ok(())
}

#[tokio::main(flavor = "multi_thread", worker_threads = 666)]
async fn main() {
    let destination_file: &'static str = "./output.txt";
    remove_file(destination_file).expect("Failed to delete destination file, to allow clean write");
    run("./BEncyclopedia.txt", destination_file)
        .await
        .expect("Failed to sanitize the wordlist");
}

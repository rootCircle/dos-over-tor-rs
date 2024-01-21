# dos-over-tor-rs
dos(not really) over tor using arti implementation of Tor protocol


Pinging multiple urls concurrently over tor using [arti](https://docs.rs/arti/latest/arti/) i.e. the implementation of tor in rust.

Not a infosec guy, so don't expect it to be perfect!

**Only for educational purposes!** I don't take any responsibility for any nonsense you do with it! Use it with your own sanity!

## Getting Started

In `src/bin/dos_over_tor.rs` in main() function, just change the base url to your required one. Also, create a `wordlist.txt` file containing list of lines that will be appended to base_url.

use clap::*;
use fair_baccarat;

fn main() {
    let matches = clap_app!(myapp =>
        (name: "fair-baccarat")
        (version: "0.1.0")
        (author: crate_authors!())
        (about: crate_description!())
        (@arg client_seed: +required "Client seed")
        (@arg server_seed: +required "Server seed")
        (@arg nonce: +required "Nonce (positive integer)")
    )
    .get_matches();

    let client_seed = matches.value_of("client_seed").unwrap();
    let server_seed = matches.value_of("server_seed").unwrap();

    let nonce: u64 = value_t!(matches, "nonce", u64).unwrap_or_else(|e| e.exit());
    // println!("{:?}", matches);
    println!("Client seed: {}", client_seed);
    println!("Server seed: {}", server_seed);
    println!("Nonce: {}", nonce);
    println!("");

    let result = fair_baccarat::simulate(client_seed, server_seed, nonce);

    println!("{}", result);
}

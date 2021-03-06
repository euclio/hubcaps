extern crate env_logger;
extern crate hyper;
extern crate hubcaps;
extern crate hyper_native_tls;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hubcaps::{Credentials, Github};
use std::env;

fn main() {
    env_logger::init().unwrap();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let github =
                Github::new(
                    format!("hubcaps/{}", env!("CARGO_PKG_VERSION")),
                    Client::with_connector(HttpsConnector::new(NativeTlsClient::new().unwrap())),
                    Credentials::Token(token),
                );
            for file in github
                .repo("softprops", "hubcaps")
                .git()
                .tree("master", true)
                .unwrap()
                .tree
                .iter()
                .find(|file| file.path == "README.md")
            {
                let blob = github
                    .repo("softprops", "hubcaps")
                    .git()
                    .blob(file.sha.clone())
                    .unwrap();
                println!("readme {:#?}", blob);
            }
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}

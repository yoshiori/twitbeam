extern crate hyper;
extern crate hyper_native_tls;

use std::env;
use std::io::{self, Write, Read};

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::header::{Authorization, Bearer};

struct TwitBeam {
    access_token: String,
    api_server: String,
    client: Client,
}

impl TwitBeam {
    fn new(access_token: &str, api_server: &str) -> TwitBeam {
        let tls = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(tls);
        TwitBeam {
            access_token: access_token.to_string(),
            api_server: api_server.to_string(),
            client: Client::with_connector(connector),
        }
    }

    fn toot(&self, text: &str) {
        self.client.post(&format!("{}/api/v1/statuses", &self.api_server))
            .header(
                Authorization(
                        Bearer {
                            token: self.access_token.clone(),
                        }
                )
            ).body(&format!("status={}", text)).send().unwrap();
    }
}

fn main() {

    let token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN not set.");
    let api_server = env::var("API_SERVER").expect("API_SERVER not set.");

    let twite_beam = TwitBeam::new(&token, &api_server);

    let mut text = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut text)
            .expect("Failed to read line");

    twite_beam.toot(&text);
}

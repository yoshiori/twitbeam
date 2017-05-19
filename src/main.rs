extern crate hyper;
extern crate hyper_native_tls;

use std::env;
use std::io::{self, Write};

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::header::{Authorization, Bearer};

fn main() {

    let token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN not set.");
    let api_server = env::var("API_SERVER").expect("API_SERVER not set.");
    let tls = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(tls);
    let client = Client::with_connector(connector);

    let mut text = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut text)
            .expect("Failed to read line");

    let res = client.post(&format!("{}/api/v1/statuses", &api_server))
        .header(
            Authorization(
                    Bearer {
                        token: token.clone(),
                    }
            )
        ).body(&format!("status={}", &text)).send();
    match res {
        Ok(res) => println!("Response: {}", res.status),
        Err(e) => println!("Err: {:?}", e)
    }
}

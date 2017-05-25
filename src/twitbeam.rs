extern crate hyper;
extern crate hyper_native_tls;
extern crate serde_json;

use std::io::Read;

use self::hyper::net::HttpsConnector;
use self::hyper::header::{Authorization, Bearer};
use self::hyper::client::RequestBuilder;
use self::hyper::client::response::Response;

use self::hyper_native_tls::NativeTlsClient;

use self::serde_json::Value;

pub struct Client {
    access_token: String,
    api_server: String,
    client: hyper::Client,
}

impl Client {
    pub fn new(access_token: &str, api_server: &str) -> Client {
        let tls = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(tls);
        Client {
            access_token: access_token.to_string(),
            api_server: api_server.to_string(),
            client: hyper::Client::with_connector(connector),
        }
    }

    pub fn toot(&self, text: &str) {
        self.send(
            self.client.post(&format!("{}/api/v1/statuses", &self.api_server))
                .body(&format!("status={}", text))
        ).unwrap();
    }

    pub fn home(&self) -> Value {
        let mut res = self.send(
            self.client.get(&format!("{}/api/v1/timelines/home", &self.api_server))
        ).unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        serde_json::from_str(&body).unwrap()
    }

    fn send(&self, req: RequestBuilder) -> hyper::error::Result<Response> {
        req.header(
            Authorization(
                    Bearer {
                        token: self.access_token.clone(),
                    }
            )
        ).send()
    }
}

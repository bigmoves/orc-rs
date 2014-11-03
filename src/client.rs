use curl::http;

use std::collections::HashMap;
use serialize::json::{Decoder, DecoderError};
use serialize::Decodable;
use serialize::json;
use serialize::base64::{MIME, ToBase64};

pub struct Client {
    api_endpoint: String,
    token: String,
    handle: http::Handle
}

pub type Headers = HashMap<String, String>;

impl Client {
    pub fn new(token: &str) -> Client {
        Client {
            api_endpoint: "api.orchestrate.io".to_string(),
            token: token.to_string(),
            handle: http::handle()
        }
    }

    pub fn from_value<T: Decodable<Decoder,DecoderError>>(value: HashMap<String, String>) -> Option<T> {
        match json::decode(json::encode(&value).as_slice()) {
            Ok(e) => Some(e),
            _ => None
        }
    }

    pub fn ping(&mut self) -> http::Response {
        self.request(http::handle::Head, "", None, None)
    }

    pub fn get_request(&mut self, path: &str) -> http::Response {
        self.request(http::handle::Get, path, None, None)
    }

    pub fn put_request(&mut self, path: &str, headers: Option<Headers>, body: &str) -> http::Response {
        self.request(http::handle::Put, path, headers, Some(body))
    }

    pub fn post_request(&mut self, path: &str, body: &str) -> http::Response {
        self.request(http::handle::Post, path, None, Some(body))
    }

    pub fn delete_request(&mut self, path: &str) -> http::Response {
        self.request(http::handle::Delete, path, None, None)
    }

    pub fn request(&mut self, method: http::handle::Method, path: &str, headers: Option<Headers>, body: Option<&str>) -> http::Response {
        let uri = format!("https://{:s}/v0/{:s}", self.api_endpoint.as_slice(), path);
        let mut request = http::handle::Request::new(&mut self.handle, method).uri(uri);

        // set basic auth header
        let mut config = MIME;
        config.line_length = None;
        request = request.header("Authorization", format!("Basic {}", self.token.as_bytes().to_base64(config).as_slice()).as_slice());

        match headers {
            Some(headers) => {
                for (name, val) in headers.iter() {
                    request = request.header(name.as_slice(), val.as_slice());
                }
            },
            None => ()
        }

        request = request.header("User-Agent", "orc-rs");

        let mut bs;
        match body {
            Some(body) => {
                bs = body.to_string();
                request = request.body(&bs);
                request = request.header("Content-Type", "application/json");
            },
            None => ()
        }

        request.exec().unwrap()
    }

}


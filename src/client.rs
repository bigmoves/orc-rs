use search::{SearchBuilder};
use events::EventReader;
use error::Error;
use std::fmt;
use std::io::IoError;
use std::collections::HashMap;
use hyper::{Url, HttpError, HttpResult};
use hyper::client::{Request};
use hyper::client::Response as HyperResponse;
use hyper::method::{Method, Head};
use hyper::header::{Header, HeaderFormat, Headers};
use hyper::header::common::{UserAgent, ContentType, ContentLength};
use hyper::status::StatusCode;
use serialize::base64::{MIME, ToBase64};

pub fn version() -> String {
    format!("orc-rs {}", format!("{}.{}.{}",
                                 env!("CARGO_PKG_VERSION_MAJOR"),
                                 env!("CARGO_PKG_VERSION_MINOR"),
                                 env!("CARGO_PKG_VERSION_PATCH")))
}

pub struct Client {
    pub api_endpoint: String,
    token: String,
    user_agent: String
}

impl Client {
    pub fn new(token: &str) -> Client {
        Client {
            api_endpoint: "api.orchestrate.io".to_string(),
            token: token.to_string(),
            user_agent: version()
        }
    }

    pub fn search<'a, 'b>(&'a mut self, collection: &str)
                          -> SearchBuilder<'a, 'b> {
        SearchBuilder::new(self, collection)
    }

    pub fn event<'a, 'b>(&'a mut self) -> EventReader<'a, 'b> {
        EventReader::new(self)
    }

    pub fn ping(&self) -> Result<bool, Error> {
        let res = self.request(Head, "", None, None).unwrap();

        if res.code != 200 {
            return Err(Error::new(res.body.as_slice()));
        }

        Ok(true)
    }

    // temporary fix
    pub fn delete_request(&self, url: &str) -> HttpResult<HyperResponse> {
        let uri = format!("https://{}/v0/{}", self.api_endpoint.as_slice(), url);
        let mut req = Request::delete(Url::parse(uri.as_slice()).unwrap()).unwrap();

        req.headers_mut().set(UserAgent(self.user_agent.to_string()));
        req.headers_mut().set(Authorization(self.token.to_string()));

        Ok(req.start().unwrap()
           .send().unwrap())
    }

    pub fn request(&self, method: Method, url: &str,
                   headers: Option<HashMap<String, String>>,
                   body: Option<&str>) -> Result<Response, ClientError> {
        let uri = format!("https://{:s}/v0/{:s}",
                          self.api_endpoint.as_slice(), url);

        let url = Url::parse(uri.as_slice()).unwrap();

        let mut req = match Request::new(method, url) {
            Ok(req) => req,
            Err(err) => return Err(HttpRequestError(err))
        };

        match headers {
            Some(hdrs) => {
                for (k, v) in hdrs.iter() {
                    req.headers_mut()
                       .set_raw(k.to_string(),
                                vec![v.to_string().as_bytes().to_vec()]);
                }
            },
            None => ()
        }

        req.headers_mut().set(UserAgent(self.user_agent.to_string()));
        req.headers_mut().set(Authorization(self.token.to_string()));

        match body {
            Some(body) => {
                req.headers_mut().set(ContentLength(body.len()));
                req.headers_mut().set(ContentType(from_str("application/json").unwrap()));
            },
            None => req.headers_mut().set(ContentLength(0))
        };

        let mut stream = match req.start() {
            Ok(stream) => stream,
            Err(err) => return Err(HttpRequestError(err))
        };

        match body {
            Some(body) => match stream.write(body.as_bytes()) {
                Ok(()) => (),
                Err(err) => return Err(HttpIoError(err))
            },
            None => ()
        };

        let mut resp = match stream.send() {
            Ok(resp) => resp,
            Err(err) => return Err(HttpRequestError(err))
        };

        let body = match resp.read_to_string() {
            Ok(body) => body,
            Err(err) => return Err(HttpIoError(err))
        };

        Ok(Response {
            code: resp.status as i32,
            status: resp.status,
            headers: resp.headers,
            body: body
        })
    }
}

pub struct Response {
    pub code: i32,
    pub status: StatusCode,
    pub headers: Headers,
    pub body: String
}

struct Authorization(String);

impl Header for Authorization {
    fn header_name(_: Option<Authorization>) -> &'static str {
        "Authorization"
    }
    fn parse_header(_: &[Vec<u8>]) -> Option<Authorization> {
        None
    }
}

impl HeaderFormat for Authorization {
    fn fmt_header(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut config = MIME;
        config.line_length = None;

        let Authorization(ref value) = *self;
        write!(fmt, "Basic {}", value.as_bytes().to_base64(config))
    }
}

#[deriving(Show, PartialEq, Clone)]
pub enum ClientError {
    HttpRequestError(HttpError),
    HttpIoError(IoError)
}

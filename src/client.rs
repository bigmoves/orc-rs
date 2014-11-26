use hyper::client;
use hyper::client::Response;
use hyper::method::Method;
use hyper::header::common::{
    UserAgent, ContentType, ContentLength, Authorization
};
use hyper::header::common::authorization::Basic;
use error::OrchestrateError;
use std::collections::HashMap;
use hyper::Url;
use url::form_urlencoded::serialize_owned;

#[deriving(Clone, Show)]
pub struct Client {
    pub host: String,
    token: String,
    user_agent: String,
    url: Option<Url>,
    method: Option<Method>,
    content_type: String,
    headers: HashMap<String, String>,
    query: Vec<(String, String)>,
    body: Option<String>
}

impl Client {

    pub fn new(token: &str) -> Client {
        Client {
            host: "api.orchestrate.io".to_string(),
            token: token.to_string(),
            user_agent: version(),
            url: None,
            method: None,
            content_type: "application/json".to_string(),
            headers: HashMap::new(),
            query: Vec::new(),
            body: None
        }
    }

    pub fn trailing(&mut self, url: &str) -> &mut Client {
        self.url = Some(Url::parse(format!("https://{:s}/v0/{:s}",
                                           self.host.as_slice(),
                                           url.as_slice()).as_slice()).unwrap());
        self
    }

    pub fn header(&mut self, name: &str, value: &str) -> &mut Client {
        self.headers.insert(name.to_string(), value.to_string());
        self
    }

    pub fn query(&mut self, name: &str, value: &str) -> &mut Client {
        self.query.push((name.to_string(), value.to_string()));
        self
    }

    pub fn body(&mut self, body: &str) -> &mut Client {
        self.body = Some(body.to_string());
        self
    }

    pub fn method(&mut self, method: Method) -> &mut Client {
        self.method = Some(method);
        self
    }

    pub fn exec(&self) -> Result<Response, OrchestrateError> {
        let mut url = self.url.clone().unwrap();

        if !self.query.is_empty() {
          url.query = Some(serialize_owned(self.query.as_slice()));
        }
        println!("{}", url);
        let mut req = try!(client::Request::new(self.method.clone().unwrap(), url));

        {
            let mut headers = req.headers_mut();
            headers.set(UserAgent(self.user_agent.to_string()));
            headers.set(Authorization(Basic {
                username: self.token.to_string(),
                password: None
            }));

            for (name, value) in self.headers.iter() {
                headers.set_raw(name.to_string(), vec![value.as_bytes().to_vec()]);
            }

            match self.body.clone() {
                Some(body) => {
                    headers.set(ContentLength(body.len()));
                    headers.set(ContentType(from_str("application/json").unwrap()));
                },
                None => headers.set(ContentLength(0))
            }
        }

        let mut stream = try!(req.start());
        if self.body.is_some() {
            try!(stream.write(self.body.clone().unwrap().as_bytes()));
        }
        Ok(try!(stream.send()))
    }
}

pub fn version() -> String {
    format!("orc-rs {}", format!("{}.{}.{}",
                                 env!("CARGO_PKG_VERSION_MAJOR"),
                                 env!("CARGO_PKG_VERSION_MINOR"),
                                 env!("CARGO_PKG_VERSION_PATCH")))
}

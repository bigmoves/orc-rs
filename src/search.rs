use client::Client;
use path::Path;
use error::{OrchestrateError, ResponseError};
use RepresentsJSON;
use serialize::json;
use hyper::method::Get;

#[deriving(Decodable, Encodable, Show)]
pub struct SearchResults<T> {
    pub count: u64,
    pub total_count: u64,
    pub results: Vec<SearchResult<T>>,
    pub next: Option<String>,
    pub prev: Option<String>
}

#[deriving(Decodable, Encodable, Show)]
pub struct SearchResult<T> {
    pub path: Path,
    pub score: f64,
    pub distance: f64,
    pub value: T
}

pub type SearchResponse<T> = Result<SearchResults<T>, OrchestrateError>;

pub struct SearchBuilder<'a> {
    client: &'a mut Client,
    url: String
}

impl<'a> SearchBuilder<'a> {
    pub fn new<'a>(client: &'a mut Client, collection: &str)
                   -> SearchBuilder<'a> {
        SearchBuilder {
            client: client,
            url: collection.to_string()
        }
    }

    pub fn limit(mut self, limit: int) -> SearchBuilder<'a> {
        self.client.query("limit", limit.to_string().as_slice());
        self
    }

    pub fn offset(mut self, offset: int) -> SearchBuilder<'a> {
        self.client.query("offset", offset.to_string().as_slice());
        self
    }

    pub fn sort(mut self, prop: &str, sort: &str) -> SearchBuilder<'a> {
        self.client.query("sort", format!("value.{}:{}", prop, sort).as_slice());
        self
    }

    pub fn query(mut self, query: &str) -> SearchBuilder<'a> {
        self.client.query("query", query);
        self
    }

    pub fn get_next<T: RepresentsJSON>(mut self, results: &SearchResults<T>)
                    -> SearchBuilder<'a> {
        match results.next {
            Some(ref next) => {
                self.url = next.slice_chars(4, next.len()).to_string();
                self
            },
            None => self
        }
    }

    pub fn get_prev<T: RepresentsJSON>(mut self, results: &SearchResults<T>)
                    -> SearchBuilder<'a> {
        match results.prev {
            Some(ref prev) => {
                self.url = prev.slice_chars(4, prev.len()).to_string();
                self
            },
            None => self
        }
    }

    pub fn exec<T: RepresentsJSON>(self) -> SearchResponse<T> {
        let SearchBuilder { client, url } = self;
        let mut res = try!(client.trailing(url.as_slice()).method(Get).exec());
        let body = try!(res.read_to_string());

        if (res.status as i32) != 200 {
            return Err(ResponseError(body));
        }

        Ok(try!(json::decode::<SearchResults<T>>(body.as_slice())))
    }
}

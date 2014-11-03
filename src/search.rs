use url;
use std::str;
use std::collections::HashMap;
use serialize::json;

use client::Client;
use path::Path;
use error::Error;

#[deriving(Decodable, Encodable)]
pub struct SearchResults {
    count: u64,
    total_count: u64,
    results: Vec<SearchResult>,
    next: Option<String>,
    prev: Option<String>
}

#[deriving(Decodable, Encodable)]
pub struct SearchResult {
    path: Path,
    score: f64,
    distance: f64,
    value: HashMap<String, String>
}

pub trait Search {
    fn search(&mut self, collection: &str, query: &str, limit: int, offset: int) -> Result<SearchResults, Error>;

    fn search_sorted(&mut self, collection: &str, query: &str, sort: &str, limit: int, offset: int) -> Result<SearchResults, Error>;

    fn exec_search(&mut self, trailing_uri: &str) -> Result<SearchResults, Error>;
}

impl Search for Client {
    fn search(&mut self, collection: &str, query: &str, limit: int, offset: int) -> Result<SearchResults, Error> {
        let query_params = [
            ("query".to_string(), query.to_string()),
            ("limit".to_string(), limit.to_string()),
            ("offset".to_string(), offset.to_string())
        ];
        let encoded_params = url::form_urlencoded::serialize_owned(query_params.as_slice());
        let trailing_uri = format!("{}?{}", collection, encoded_params.as_slice());
        self.exec_search(trailing_uri.as_slice())
    }

    fn search_sorted(&mut self, collection: &str, query: &str, sort: &str, limit: int, offset: int) -> Result<SearchResults, Error> {
        let query_params = [
            ("query".to_string(), query.to_string()),
            ("limit".to_string(), limit.to_string()),
            ("offset".to_string(), offset.to_string()),
            ("sort".to_string(), sort.to_string())
        ];
        let encoded_params = url::form_urlencoded::serialize_owned(query_params.as_slice());
        let trailing_uri = format!("{}?{}", collection, encoded_params.as_slice());
        self.exec_search(trailing_uri.as_slice())
    }

    fn exec_search(&mut self, trailing_uri: &str) -> Result<SearchResults, Error> {
        let res = self.get_request(trailing_uri);

        if res.get_code() != 200 {
            return Err(Error::new(res));
        }

        let results = str::from_utf8(res.get_body());

        Ok(json::decode(results.unwrap()).unwrap())
    }
}

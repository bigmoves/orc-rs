use client::Client;
use error::Error;
use path::Path;

use url::form_urlencoded::serialize_owned as serialize;
use hyper::Get;

use serialize::json;
use RepresentsJSON;

#[deriving(Decodable, Encodable, Show)]
pub struct SearchResults<T> {
    pub count: u64,
    pub total_count: u64,
    pub results: Vec<SearchResult<T>>,
    pub next: Option<String>,
    pub prev: Option<String>
}

impl<T> SearchResults<T> {
    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }

    pub fn has_prev(&self) -> bool {
        self.prev.is_some()
    }
}

#[deriving(Decodable, Encodable, Show)]
pub struct SearchResult<T> {
    pub path: Path,
    pub score: f64,
    pub distance: f64,
    pub value: T
}

pub struct SearchBuilder<'a, 'b> {
    collection: String,
    query: Option<String>,
    sort: Vec<String>,
    limit: Option<int>,
    offset: Option<int>,
    client: &'a mut Client
}

impl<'a, 'b> SearchBuilder<'a, 'b> {
    pub fn new<'a, 'b>(client: &'a mut Client, collection: &str)
                       -> SearchBuilder<'a, 'b> {
        SearchBuilder{
            collection: collection.to_string(),
            query: None,
            sort: Vec::new(),
            limit: None,
            offset: None,
            client: client
        }
    }

    pub fn limit(mut self, limit: int) -> SearchBuilder<'a, 'b> {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: int) -> SearchBuilder<'a, 'b> {
        self.offset = Some(offset);
        self
    }

    pub fn sort(mut self, prop: &str, sort: &str) -> SearchBuilder<'a, 'b> {
        self.sort.push(format!("value.{}:{}", prop, sort));
        self
    }

    pub fn query(mut self, query: &str) -> SearchBuilder<'a, 'b> {
        self.query = Some(query.to_string());
        self
    }

    pub fn get_next<T: RepresentsJSON>(&self, results: SearchResults<T>)
                    -> Result<SearchResults<T>, Error> {
        let next = results.next.unwrap();
        let uri = next.slice_chars(4, next.len());
        let res = self.client.request(Get, uri, None, None).unwrap();

        if res.code != 200 {
            return Err(Error::new(res.body.as_slice()));
        }

        Ok(json::decode::<SearchResults<T>>(res.body.as_slice()).unwrap())
    }

    pub fn get_prev<T: RepresentsJSON>(&self, results: SearchResults<T>)
                    -> Result<SearchResults<T>, Error> {
        let prev = results.prev.unwrap();
        let uri = prev.slice_chars(4, prev.len());
        let res = self.client.request(Get, uri, None, None).unwrap();

        if res.code != 200 {
            return Err(Error::new(res.body.as_slice()));
        }

        Ok(json::decode::<SearchResults<T>>(res.body.as_slice()).unwrap())
    }

    pub fn exec<T: RepresentsJSON>(self) -> Result<SearchResults<T>, Error> {
        let SearchBuilder {
            collection,
            query,
            sort,
            limit,
            offset,
            client,
            ..
        } = self;

        let mut query_params: Vec<(String, String)> = Vec::new();

        match limit {
            Some(l) => query_params.push(("limit".to_string(), l.to_string())),
            None => {}
        }

        match offset {
            Some(o) => query_params.push(("offset".to_string(), o.to_string())),
            None => {}
        }

        match query {
            Some(q) => query_params.push(("query".to_string(), q)),
            None => {}
        }

        for item in sort.iter() {
            query_params.push(("sort".to_string(), item.clone()));
        }

        let encoded_params = serialize(query_params.as_slice());
        let trailing_uri = format!("{}?{}", collection.as_slice(),
                                   encoded_params.as_slice());
        let res = client.request(Get, trailing_uri.as_slice(), None,
                                     None).unwrap();

        if res.code != 200 {
            return Err(Error::new(res.body.as_slice()));
        }

        Ok(json::decode::<SearchResults<T>>(res.body.as_slice()).unwrap())
    }
}


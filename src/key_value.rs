use client::Client;
use path::Path;
use error::Error;
use std::io::IoError;
use hyper::method::{Get, Put, Post, Delete};
use hyper::header::common::location::Location;
use url::form_urlencoded::serialize_owned as serialize;
use serialize::{json, Encodable};
use serialize::json::Encoder;
use RepresentsJSON;

#[deriving(Decodable, Encodable, Show)]
pub struct KVResults<T> {
    pub count: int,
    pub results: Vec<KVResult<T>>,
    pub next: Option<String>
}

impl<T> KVResults<T> {
    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }
}

#[deriving(Decodable, Encodable, Show)]
pub struct KVResult<T> {
    pub path: Path,
    pub value: T
}

pub trait KeyValue {
    fn get<T: RepresentsJSON>(&self, collection: &str, key: &str)
                              -> Result<KVResult<T>, Error>;

    fn post<'a,
            T: Encodable<Encoder<'a>, IoError>>(
                 &self,
                 collection: &str,
                 value: &T) -> Result<Path, Error>;

    fn put<'a,
           T: Encodable<Encoder<'a>, IoError>>(
               &self,
               collection: &str,
               key: &str, value: &T) -> Result<Path, Error>;

    fn delete(&self, collection: &str, key: &str) -> Result<bool, Error>;
    fn purge(&self, collection: &str, key: &str) -> Result<bool, Error>;
    fn list<'a, 'b>(&'a mut self, collection: &str) -> ListReader<'a, 'b>;
}

impl KeyValue for Client {
    fn get<T: RepresentsJSON>(&self, collection: &str, key: &str)
                              -> Result<KVResult<T>, Error> {
        let uri = format!("{}/{}", collection.to_string(), key.to_string());
        let res = self.request(Get, uri.as_slice(), None, None).unwrap();

        if res.code != 200 {
            return Err(Error::new(res.body.as_slice()));
        }

        Ok(KVResult {
            path: Path {
                collection: collection.to_string(),
                key: key.to_string(),
                ref_: None
            },
            value: json::decode::<T>(res.body.as_slice()).unwrap()
        })
    }

    fn post<'a,
            T: Encodable<Encoder<'a>, IoError>>(
                &self,
                collection: &str,
                value: &T) -> Result<Path, Error> {
        let json_str = json::encode(value);
        let res = self.request(Post, collection, None,
                                   Some(json_str.as_slice())).unwrap();

        if res.code != 201 {
            return Err(Error::new(res.body.as_slice()));
        }

        let Location(ref location) = *res.headers.get::<Location>().unwrap();
        let parts: Vec<&str> = location.split('/').collect();
        let key = parts[3].to_string();
        let ref_ = Some(parts[5].to_string());

        Ok(Path {
            collection: collection.to_string(),
            key: key,
            ref_: ref_
        })
    }

    fn put<'a,
           T: Encodable<Encoder<'a>, IoError>>(
               &self,
               collection: &str,
               key: &str, value: &T) -> Result<Path, Error> {
        let json_str = json::encode(&value);
        let trailing_uri = format!("{}/{}", collection.as_slice(),
                                   key.as_slice());
        let res = self.request(Put, trailing_uri.as_slice(), None,
                               Some(json_str.as_slice())).unwrap();

        if res.code != 201 {
            return Err(Error::new(res.body.as_slice()));
        }

        let Location(ref location) = *res.headers.get::<Location>().unwrap();
        let parts: Vec<&str> = location.split('/').collect();
        let ref_ = Some(parts[5].to_string());

        Ok(Path {
            collection: collection.to_string(),
            key: key.to_string(),
            ref_: ref_
        })
    }

    fn delete(&self, collection: &str, key: &str) -> Result<bool, Error> {
        let uri = format!("{}/{}", collection.to_string(),
                          key.to_string());
        let mut res = self.delete_request(uri.as_slice()).unwrap();

        if (res.status as i32) != 204 {
            return Err(Error::new(res.read_to_string().unwrap().as_slice()));
        }

        Ok(true)
    }

    fn purge(&self, collection: &str, key: &str) -> Result<bool, Error> {
        let uri = format!("{}/{}?purge=true", collection.to_string(),
                          key.to_string());
        let mut res = self.delete_request(uri.as_slice()).unwrap();

        if (res.status as i32) != 204 {
            return Err(Error::new(res.read_to_string().unwrap().as_slice()));
        }

        Ok(true)
    }

    fn list<'a, 'b>(&'a mut self, collection: &str) -> ListReader<'a, 'b> {
        ListReader::new(self, collection)
    }
}

pub struct ListReader<'a, 'b> {
    collection: String,
    limit: Option<int>,
    start_key: Option<String>,
    after_key: Option<String>,
    before_key: Option<String>,
    end_key: Option<String>,
    client: &'a mut Client
}

impl<'a, 'b> ListReader<'a, 'b> {
    pub fn new<'a, 'b>(client: &'a mut Client, collection: &str)
                       -> ListReader<'a, 'b> {
        ListReader {
            collection: collection.to_string(),
            limit: None,
            start_key: None,
            after_key: None,
            before_key: None,
            end_key: None,
            client: client
        }
    }

    pub fn limit(mut self, limit: int) -> ListReader<'a, 'b> {
        self.limit = Some(limit);
        self
    }

    pub fn start_key(mut self, start_key: &str) -> ListReader<'a, 'b> {
        self.start_key = Some(start_key.to_string());
        self
    }

    pub fn after_key(mut self, after_key: &str) -> ListReader<'a, 'b> {
        self.after_key = Some(after_key.to_string());
        self
    }

    pub fn before_key(mut self, before_key: &str) -> ListReader<'a, 'b> {
        self.before_key = Some(before_key.to_string());
        self
    }

    pub fn end_key(mut self, end_key: &str) -> ListReader<'a, 'b> {
        self.end_key = Some(end_key.to_string());
        self
    }

    pub fn exec<T: RepresentsJSON>(self) -> Result<KVResults<T>, Error> {
        let ListReader {
            collection,
            limit,
            start_key,
            after_key,
            before_key,
            end_key,
            client,
            ..
        } = self;

        let mut query_params: Vec<(String, String)> = Vec::new();

        match limit {
            Some(l) => query_params.push(("limit".to_string(), l.to_string())),
            None => {}
        }

        match start_key {
            Some(sk) => query_params.push(("startKey".to_string(),
                                           sk.to_string())),
            None => {}
        }

        match after_key {
            Some(ak) => query_params.push(("afterKey".to_string(),
                                          ak.to_string())),
            None => {}
        }

        match before_key {
            Some(bk) => query_params.push(("beforeKey".to_string(),
                                          bk.to_string())),
            None => {}
        }

        match end_key {
            Some(ek) => query_params.push(("endKey".to_string(),
                                          ek.to_string())),
            None => {}
        }

        let encoded_params = serialize(query_params.as_slice());
        let trailing_uri = format!("{}?{}", collection.as_slice(),
                                   encoded_params.as_slice());
        let res = client.request(Get, trailing_uri.as_slice(), None,
                                     None).unwrap();

        if res.code != 200 {
            return Err(Error::new(res.body.as_slice()));
        }

        Ok(json::decode::<KVResults<T>>(res.body.as_slice()).unwrap())
    }
}


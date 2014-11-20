#![crate_name="orchestrate"]

#![feature(globs, phase)]

extern crate serialize;
extern crate hyper;
extern crate url;

pub use client::Client;
pub use key_value::{
    GetKeyValue, CreateKeyValue, UpdateKeyValue, DeleteKeyValue, KeyValueResult,
    KeyValueResults, PathResult, ListReader
};
pub use search::{SearchBuilder, SearchResults, SearchResult};
pub use events::{
  GetEvents, CreateEvent, DeleteEvent, EventResults, EventResult
};
pub use error::{OrchestrateError, ResponseError};
pub use path::Path;
use serialize::{json, Decoder, Decodable};
use hyper::method::Head;

pub trait RepresentsJSON : Decodable<json::Decoder, json::DecoderError> {}
impl<T: Decodable<json::Decoder, json::DecoderError>> RepresentsJSON for T {}

pub struct Orchestrate {
    client: Client
}

impl Orchestrate {

    // create a new orchestrate client
    pub fn new(token: &str) -> Orchestrate {
        Orchestrate {
            client: Client::new(token)
        }
    }

    pub fn ping(&mut self) -> Result<bool, OrchestrateError> {
        let mut res = try!(self.client.trailing("").method(Head).exec());

        if (res.status as i32) != 200 {
            return Err(ResponseError(try!(res.read_to_string())));
        }

        Ok(true)
    }

    pub fn get<'a>(&'a mut self, collection: &str, key: &str)
                    -> GetKeyValue<'a> {
        GetKeyValue::new(&mut self.client, collection, key)
    }

    pub fn post<'a>(&'a mut self, collection: &str) -> CreateKeyValue<'a> {
        CreateKeyValue::new(&mut self.client, collection)
    }

    pub fn put<'a>(&'a mut self, collection: &str, key: &str)
                   -> UpdateKeyValue<'a> {
        UpdateKeyValue::new(&mut self.client, collection, key)
    }

    pub fn delete<'a>(&'a mut self, collection: &str, key: &str)
                      -> DeleteKeyValue<'a> {
        DeleteKeyValue::new(&mut self.client, collection, key)
    }

    pub fn list<'a>(&'a mut self, collection: &str) -> ListReader<'a> {
        ListReader::new(&mut self.client, collection)
    }

    pub fn search<'a>(&'a mut self, collection: &str) -> SearchBuilder<'a> {
        SearchBuilder::new(&mut self.client, collection)
    }

    pub fn get_events<'a>(&'a mut self, collection: &str, key: &str, kind: &str)
                          -> GetEvents<'a> {
        GetEvents::new(&mut self.client, collection, key, kind)
    }

    pub fn create_event<'a>(&'a mut self, collection: &str, key: &str,
                            kind: &str) -> CreateEvent<'a> {
        CreateEvent::new(&mut self.client, collection, key, kind)
    }

    pub fn delete_event<'a>(&'a mut self, collection: &str, key: &str,
                            kind: &str) -> DeleteEvent<'a> {
        DeleteEvent::new(&mut self.client, collection, key, kind)
    }
}

mod client;
mod key_value;
mod search;
mod events;
mod error;
mod path;

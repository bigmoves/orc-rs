#![crate_name="orchestrate"]

#![feature(globs, phase)]

extern crate serialize;
extern crate hyper;
extern crate url;

pub use error::OrchestrateError;

use client::Client;
use key_value::{
    GetKeyValue, CreateKeyValue, UpdateKeyValue, DeleteKeyValue, ListReader,
};
use search::SearchBuilder;
use events::{GetEvents, CreateEvent, DeleteEvent};
use graph::{GetRelations, PutRelation, DeleteRelation};
use serialize::{json, Decoder, Decodable};
use hyper::method::{Head, Delete};

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
            return Err(error::RequestError(try!(res.read_to_string())));
        }

        Ok(true)
    }

    pub fn delete_collection(&mut self, collection: &str)
                             -> Result<bool, OrchestrateError> {
        let mut res = try!(self.client.trailing(collection)
                                      .query("force", "true")
                                      .method(Delete)
                                      .exec());

        if (res.status as i32) != 204 {
            return Err(error::RequestError(try!(res.read_to_string())));
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

    pub fn get_relations<'a>(&'a mut self, collection: &str, key: &str,
                             hops: Vec<&str>) -> GetRelations<'a> {
        GetRelations::new(&mut self.client, collection, key, hops)
    }

    pub fn put_relation<'a>(&'a mut self, collection: &str, key: &str,
                            kind: &str, to_collection: &str, to_key: &str)
                            -> PutRelation<'a> {
        PutRelation::new(&mut self.client, collection, key, kind, to_collection,
                         to_key)
    }

    pub fn delete_relation<'a>(&'a mut self, collection: &str, key: &str,
                            kind: &str, to_collection: &str, to_key: &str)
                            -> DeleteRelation<'a> {
        DeleteRelation::new(&mut self.client, collection, key, kind,
                            to_collection, to_key)
    }
}

mod client;
mod error;
mod path;

pub mod key_value;
pub mod search;
pub mod events;
pub mod graph;

#![crate_name="orchestrate"]

#![feature(globs)]
#![feature(phase)]

extern crate serialize;
extern crate hyper;
extern crate url;

pub use client::{Client};
pub use key_value::{KeyValue, KVResult, KVResults, ListReader};
pub use search::SearchBuilder;
pub use events::EventReader;
pub use error::Error;
pub use path::Path;

use serialize::{Decoder, Decodable, json};

mod client;
mod key_value;
mod search;
mod events;
mod error;
mod path;

pub trait RepresentsJSON : Decodable<json::Decoder, json::DecoderError> {}
impl<T: Decodable<json::Decoder, json::DecoderError>> RepresentsJSON for T {}

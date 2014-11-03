#![crate_name="orchestrate"]
#![feature(macro_rules)]

#![feature(phase)]
#![feature(globs)]

extern crate curl;
extern crate serialize;
extern crate url;

pub use client::{Client, Headers};
pub use key_value::{KV, KVResult, KVResults};
pub use search::Search;
pub use error::Error;
pub use path::Path;

mod client;
mod key_value;
mod search;
mod error;
mod path;

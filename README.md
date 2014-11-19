orc-rs
======

A Rust client for Orchestrate.io

**HTTP clients for Rust are in active development and this could break at any
time.**

Examples:

```rust
// Import the client
extern crate orchestrate;
extern crate sesrialize;

use orchestrate::Orchestrate;
use serialize::json;

// Create a client
let mut client = Orchestrate::new("API Key");

// Create a data structure
#[deriving(Encodable, Decodable)]
struct User {
    name: String,
    email: String
}

// Get a value with pattern matching
match client.get("users", "key").exec::<User>() {
    Ok(result) => {
      println!("{}", result);
      println!("{}", json::encode(&result)); // serialize the result as JSON
    },
    Err(err) => println!("{}", err.description())
}

// or unwrap the result
let result = client.get("users", "key").exec<User>().unwrap();

// Search
let results = client.search("users")
                    .limit(10)
                    .sort("name", "desc")
                    .query("chad")
                    .exec::<User>().unwrap();

// Get the next page of results
if results.has_next() {
    let next_results = client.search("users")
                             .get_next(&results)
                             .exec::<User>().unwrap();
    println!("{}", json::encode(&next_results));
}

// Events

#[deriving(Encodable, Decodable)]
struct Update {
    msg: String
}

// Get Events
let events = client.get_events("users", "key", "update")
                   .exec::<Update>().unwrap();

// Create an Event
let update = Update { msg: "hello".to_string() };
let event = client.create_event("users", "key", "update")
                  .data(&update)
                  .exec().unwrap();

```

## Running the examples

Set your Orchestrate.io API Key as an environment variable.

```shell
export ORC_API_KEY=...
```

Build and run an example.

```shell
cargo run --example name
```

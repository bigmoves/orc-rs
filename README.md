orc-rs
======

A Rust client for Orchestrate.io

**HTTP clients for Rust are in active development and this could break at any
time.**

Examples:

```rust
extern crate orchestrate;
extern crate serialize;

use orchestrate::{Client, KeyValue};
use serialize::json;

fn main() {

  let mut client = Client::new("Your API Key");

  // define a user schema
  #[deriving(Encodable, Decobale, Show)]
  struct User {
    name: String,
    email: String,
    username: Option<String>,
    age: int
  }

  // create a user object
  let mut user = User {
    name: "Chad".to_string(),
    email: "chadtmiller15@gmail.com".to_string(),
    username: None,
    age: 25
  };

  // create a user, returns a path object with the generated key
  match client.post("users", &user) {
    Ok(result) => println!("{}", result.key),
    Err(err) => println!("{}", err.message)
  };

  // retrieve a user
  let result = client.get::<User>("users", "key").unwrap();
  println!("{}", result.value.age);
  println!("{}", result.path.key);

  // update a user
  user.username = Some("chadtmiller".to_string());
  match client.put("users", &user) {
    Ok(result) => println!("{}", result.ref_),
    Err(err) => println!("{}", err.message)
  }
}
```

## Running the examples

```shell
cargo run --examples name
```


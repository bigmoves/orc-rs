#![feature(macro_rules)]
extern crate orchestrate;
extern crate serialize;
extern crate nickel;
extern crate http;

use http::status::{NotFound, NoContent};
use nickel::{Nickel, Request, Response, HttpRouter, JsonBody, Continue,
    MiddlewareResult, QueryString
};
use nickel::mimes;
use std::io::net::ip::Ipv4Addr;
use serialize::json;
use orchestrate::{Client, KeyValue};

const ORC_API_KEY: &'static str = env!("ORC_API_KEY");

#[deriving(Encodable, Decodable)]
struct User {
    name: String,
    email: String
}

#[cfg(not(test))]
fn main() {
    let mut server = Nickel::new();

    fn logger(request: &Request, _response: &mut Response) -> MiddlewareResult {
        println!("logging request: {}", request.origin.request_uri);

        Ok(Continue)
    }

    server.utilize(logger);
    server.utilize(Nickel::json_body_parser());
    server.utilize(Nickel::query_string());

    server.get("/users/search", search_users);
    server.get("/users/:user_key", get_user);
    server.get("/users", get_users);
    server.post("/users", post_user);
    server.put("/users/:user_key", put_user);
    server.delete("/users/:user_key", delete_user);

    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}

fn search_users (req: &Request, res: &mut Response) {
    let mut client = Client::new(ORC_API_KEY);
    res.content_type(mimes::Json);

    match client.search("users")
                .limit(from_str::<int>(req.query("limit", "100")[0].as_slice()).unwrap())
                .offset(from_str::<int>(req.query("offset", "0")[0].as_slice()).unwrap())
                .query(req.query("query", "*")[0].as_slice())
                .exec::<User>() {
        Ok(results) => res.send(json::encode(&results)),
        Err(error) => res.status_code(NotFound).send(json::encode(&error))
    }
}

fn get_user (req: &Request, res: &mut Response) {
    let client = Client::new(ORC_API_KEY);

    res.content_type(mimes::Json);

    match client.get::<User>("users", req.param("user_key")) {
        Ok(result) => {
            res.send(json::encode(&result))
        },
        Err(error) => res.status_code(NotFound).send(json::encode(&error))
    }
}

fn get_users (req: &Request, res: &mut Response) {
    let mut client = Client::new(ORC_API_KEY);
    res.content_type(mimes::Json);

    match client.list("users").exec::<User>() {
        Ok(results) => {
            res.send(json::encode(&results))
        },
        Err(error) => res.status_code(NotFound).send(json::encode(&error))
    }
}

fn post_user (req: &Request, res: &mut Response) {
    let client = Client::new(ORC_API_KEY);
    let user = req.json_as::<User>().unwrap();

    res.content_type(mimes::Json);

    match client.post("users", &user) {
        Ok(result) => res.send(json::encode(&result)),
        Err(error) => res.status_code(NotFound).send(json::encode(&error))
    }
}

fn put_user (req: &Request, res: &mut Response) {
    let client = Client::new(ORC_API_KEY);
    let user = req.json_as::<User>().unwrap();

    res.content_type(mimes::Json);

    match client.put("users", req.param("user_key"), &user) {
        Ok(result) => res.send(json::encode(&result)),
        Err(error) => res.status_code(NotFound).send(json::encode(&error))
    }
}

fn delete_user (req: &Request, res: &mut Response) {
    let client = Client::new(ORC_API_KEY);

    res.content_type(mimes::Json);

    match client.delete("users", req.param("user_key")) {
        Ok(result) => res.status_code(NoContent).send(""),
        Err(error) => res.status_code(NotFound).send(json::encode(&error))
    }
}


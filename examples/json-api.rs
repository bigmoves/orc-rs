#![feature(macro_rules)]
extern crate orchestrate;
extern crate serialize;
extern crate nickel;
extern crate http;

use http::status;
use http::status::{NotFound, NoContent};
use nickel::{Nickel, Request, Response, HttpRouter, JsonBody, Continue,
    MiddlewareResult, QueryString
};
use nickel::mimes;
use std::io::net::ip::Ipv4Addr;
use serialize::json;
use orchestrate::Orchestrate;
use std::error::Error;

const ORC_API_KEY: &'static str = env!("ORC_API_KEY");

#[deriving(Encodable, Decodable)]
struct User {
    name: String,
    email: String
}

#[deriving(Encodable, Decodable)]
struct Update {
    message: String,
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
    server.get("/users/:user_key/updates", get_updates);
    server.post("/users/:user_key/updates", post_update);
    server.get("/users/:user_key", get_user);
    server.get("/users", get_users);
    server.post("/users", post_user);
    server.put("/users/:user_key", put_user);
    server.delete("/users/:user_key", delete_user);

    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}

fn get_user (req: &Request, res: &mut Response) -> (status::Status, String) {
    let mut client = Orchestrate::new(ORC_API_KEY);
    res.content_type(mimes::Json);

    match client.get("users", req.param("user_key")).exec::<User>() {
        Ok(result) => (status::Ok, json::encode(&result)),
        Err(err) => (NotFound, err.description().to_string())
    }
}

fn get_users (req: &Request, res: &mut Response) -> (status::Status, String) {
    let mut client = Orchestrate::new(ORC_API_KEY);
    res.content_type(mimes::Json);

    match client.list("users").limit(100).exec::<User>() {
        Ok(results) => (status::Ok, json::encode(&results)),
        Err(err) => (NotFound, err.description().to_string())
    }
}

fn post_user (req: &Request, res: &mut Response) -> (status::Status, String) {
    let mut client = Orchestrate::new(ORC_API_KEY);
    let user = req.json_as::<User>().unwrap();
    res.content_type(mimes::Json);

    match client.post("users").data(&user).exec() {
        Ok(result) => (status::Ok, json::encode(&result)),
        Err(err) => (status::NotFound, err.description().to_string())
    }
}

fn put_user (req: &Request, res: &mut Response) -> (status::Status, String) {
    let mut client = Orchestrate::new(ORC_API_KEY);
    let user = req.json_as::<User>().unwrap();
    res.content_type(mimes::Json);

    match client.put("users", req.param("user_key")).data(&user).if_absent().exec() {
        Ok(result) => (status::Ok, json::encode(&result)),
        Err(err) => (status::NotFound, err.description().to_string())
    }
}

fn delete_user (req: &Request, res: &mut Response) -> (status::Status, String) {
    let mut client = Orchestrate::new(ORC_API_KEY);
    res.content_type(mimes::Json);

    match client.delete("users", req.param("user_key")).purge().exec() {
        Ok(result) => (status::Ok, "".to_string()),
        Err(err) => (NotFound, err.description().to_string())
    }
}

fn search_users (req: &Request, res: &mut Response) -> (status::Status, String) {
    let mut client = Orchestrate::new(ORC_API_KEY);
    res.content_type(mimes::Json);

    match client.search("users")
                .limit(from_str::<int>(req.query("limit", "100")[0].as_slice()).unwrap())
                .offset(from_str::<int>(req.query("offset", "0")[0].as_slice()).unwrap())
                .query(req.query("query", "*")[0].as_slice())
                .exec::<User>() {
        Ok(results) => (status::Ok, json::encode(&results)),
        Err(err) => (NotFound, err.description().to_string())
    }
}

fn get_updates (req: &Request, res: &mut Response) -> (status::Status, String) {
    let mut client = Orchestrate::new(ORC_API_KEY);
    res.content_type(mimes::Json);

    match client.get_events("users", req.param("user_key"), "update")
                .limit(100)
                .exec::<Update>() {
        Ok(results) => (status::Ok, json::encode(&results)),
        Err(err) => (NotFound, err.description().to_string())
    }
}

fn post_update (req: &Request, res: &mut Response) -> (status::Status, String) {
    let mut client = Orchestrate::new(ORC_API_KEY);
    let update = req.json_as::<Update>().unwrap();
    res.content_type(mimes::Json);

    match client.create_event("users", req.param("user_key"), "update")
                .data(&update)
                .exec() {
        Ok(results) => (status::Ok, "".to_string()),
        Err(err) => (NotFound, err.description().to_string())
    }
}

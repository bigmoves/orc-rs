use client::Client;
use path::Path;
use error::{OrchestrateError, ResponseError};
use RepresentsJSON;
use serialize::{json, Encodable};
use serialize::json::Encoder;
use std::io::IoError;
use hyper::method::{Get, Post, Delete};

#[deriving(Decodable, Encodable, Show)]
pub struct EventResults<T> {
    pub count: u64,
    pub results: Vec<EventResult<T>>,
}

#[deriving(Decodable, Encodable, Show)]
pub struct EventResult<T> {
    pub ordinal: u64,
    pub timestamp: u64,
    pub value: T
}

pub type Events<T> = Result<EventResults<T>, OrchestrateError>;
pub type Event<T> = Result<EventResult<T>, OrchestrateError>;

pub struct CreateEvent<'a> {
    client: &'a mut Client,
    url: String,
    data: Option<String>,
    timestamp: Option<String>
}

impl<'a> CreateEvent<'a> {
    pub fn new(client: &'a mut Client, collection: &str, key: &str, kind: &str)
               -> CreateEvent<'a> {
        CreateEvent {
            client: client,
            url: format!("{}/{}/events/{}", collection, key, kind),
            data: None,
            timestamp: None
        }
    }

    pub fn data<'b,
                T: Encodable<Encoder<'b>, IoError>>(
                    mut self,
                    data: &T) -> CreateEvent<'a> {
        self.data = Some(json::encode(&data));
        self
    }

    pub fn timestamp(mut self, time: u64) -> CreateEvent<'a> {
        self.timestamp = Some(time.to_string());
        self
    }

    pub fn exec(self) -> Result<bool, OrchestrateError> {
        let CreateEvent { client, mut url, data, timestamp } = self;

        if timestamp.is_some() {
            let parts = vec![url, timestamp.unwrap()];
            url = parts.connect("/");
        }

        let mut res = try!(client.trailing(url.as_slice())
                                 .body(data.unwrap().as_slice())
                                 .method(Post)
                                 .exec());
        let body = try!(res.read_to_string());

        if (res.status as i32) != 201 {
            return Err(ResponseError(body));
        }

        Ok(true)
    }
}

pub struct DeleteEvent<'a> {
    client: &'a mut Client,
    url: String,
    timestamp: Option<String>,
    ordinal: Option<String>,
    ref_: Option<String>
}

impl<'a> DeleteEvent<'a> {

    pub fn new(client: &'a mut Client, collection: &str, key: &str, kind: &str)
               -> DeleteEvent<'a> {
        DeleteEvent {
            client: client,
            url: format!("{}/{}/events/{}", collection, key, kind),
            timestamp: None,
            ordinal: None,
            ref_: None
        }
    }

    pub fn timestamp(mut self, time: u64) -> DeleteEvent<'a> {
        self.timestamp = Some(time.to_string());
        self
    }

    pub fn ordinal(mut self, ordinal: u64) -> DeleteEvent<'a> {
        self.ordinal = Some(ordinal.to_string());
        self
    }

    pub fn if_match(mut self, ref_: &str) -> DeleteEvent<'a> {
        self.ref_ = Some(ref_.to_string());
        self
    }

    pub fn purge(mut self) -> DeleteEvent<'a> {
        self.client.query("purge", "true");
        self
    }

    pub fn exec(self) -> Result<bool, OrchestrateError> {
        let DeleteEvent { mut client, mut url, timestamp, ordinal, ref_ } = self;
        url = vec![url, timestamp.unwrap(), ordinal.unwrap()].connect("/");

        if ref_.is_some() {
            client.header("If-Match", ref_.unwrap().as_slice());
        }

        let mut res = try!(client.trailing(url.as_slice())
                                 .method(Delete)
                                 .exec());

        if (res.status as i32) != 204 {
            return Err(ResponseError(try!(res.read_to_string())));
        }

        Ok(true)
    }
}

pub struct GetEvents<'a> {
    client: &'a mut Client,
    url: String
}

impl<'a> GetEvents<'a> {
    pub fn new(client: &'a mut Client, collection: &str, key: &str, kind: &str)
               -> GetEvents<'a> {
        GetEvents {
            client: client,
            url: format!("{}/{}/events/{}", collection, key, kind)
        }
    }

    pub fn start(mut self, start: u64) -> GetEvents<'a> {
        self.client.query("start", start.to_string().as_slice());
        self
    }

    pub fn end(mut self, end: u64) -> GetEvents<'a> {
        self.client.query("end", end.to_string().as_slice());
        self
    }

    pub fn limit(mut self, limit: int) -> GetEvents<'a> {
        self.client.query("limit", limit.to_string().as_slice());
        self
    }

    pub fn start_event(mut self, start_event: u64) -> GetEvents<'a> {
        self.client.query("startEvent", start_event.to_string().as_slice());
        self
    }

    pub fn after_event(mut self, after_event: u64) -> GetEvents<'a> {
        self.client.query("afterEvent", after_event.to_string().as_slice());
        self
    }

    pub fn before_event(mut self, before_event: u64) -> GetEvents<'a> {
        self.client.query("beforeEvent", before_event.to_string().as_slice());
        self
    }

    pub fn end_event(mut self, end_event: u64) -> GetEvents<'a> {
        self.client.query("endEvent", end_event.to_string().as_slice());
        self
    }

    pub fn exec<T: RepresentsJSON>(self) -> Events<T> {
        let GetEvents { client, url } = self;
        let mut res = try!(client.trailing(url.as_slice()).method(Get).exec());
        let body = try!(res.read_to_string());

        if (res.status as i32) != 200 {
            return Err(ResponseError(body));
        }

        Ok(try!(json::decode::<EventResults<T>>(body.as_slice())))
    }
}

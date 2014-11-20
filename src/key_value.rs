use client::Client;
use path::Path;
use error::{OrchestrateError, ResponseError};
use RepresentsJSON;
use serialize::{json, Encodable};
use serialize::json::Encoder;
use std::io::IoError;
use hyper::header::common::location::Location;
use hyper::method::{Get, Put, Post, Delete};

#[deriving(Encodable, Decodable, Show)]
pub struct KVResult<T> {
    pub path: Path,
    pub value: T
}

#[deriving(Encodable, Decodable, Show)]
pub struct KVResults<T> {
    pub count: int,
    pub results: Vec<KVResult<T>>,
    pub next: Option<String>
}

pub type KeyValueResult<T> = Result<KVResult<T>, OrchestrateError>;
pub type KeyValueResults<T> = Result<KVResults<T>, OrchestrateError>;
pub type PathResult = Result<Path, OrchestrateError>;

pub struct GetKeyValue<'a> {
    client: &'a mut Client,
    collection: String,
    key: String,
    url: String
}

impl<'a> GetKeyValue<'a> {

    pub fn new(client: &'a mut Client, collection: &str, key: &str)
               -> GetKeyValue<'a> {
        GetKeyValue {
            client: client,
            collection: collection.to_string(),
            key: key.to_string(),
            url: format!("{}/{}", collection, key)
        }
    }

    pub fn exec<T: RepresentsJSON>(self) -> KeyValueResult<T> {
        let GetKeyValue { client, collection, key, url } = self;
        let mut res = try!(client.trailing(url.as_slice())
                                 .method(Get)
                                 .exec());
        let body = try!(res.read_to_string());

        if (res.status as i32) != 200 {
            return Err(ResponseError(body));
        }

        Ok(KVResult {
            path: Path {
                collection: collection,
                key: key,
                ref_: None
            },
            value: try!(json::decode::<T>(body.as_slice()))
        })
    }
}

pub struct CreateKeyValue<'a> {
    client: &'a mut Client,
    collection: String,
    url: String,
    data: Option<String>
}

impl<'a> CreateKeyValue<'a> {

    pub fn new(client: &'a mut Client, collection: &str) -> CreateKeyValue<'a> {
        CreateKeyValue {
            client: client,
            collection: collection.to_string(),
            url: collection.to_string(),
            data: None
        }
    }

    pub fn data<'b,
                T: Encodable<Encoder<'b>, IoError>>(
                    mut self,
                    data: &T) -> CreateKeyValue<'a> {
        self.data = Some(json::encode(&data));
        self
    }

    pub fn exec(self) -> PathResult {
        let CreateKeyValue { client, collection, url, data } = self;
        let mut res = try!(client.trailing(url.as_slice())
                                 .body(data.unwrap().as_slice())
                                 .method(Post)
                                 .exec());
        let body = try!(res.read_to_string());

        if (res.status as i32) != 201 {
            return Err(ResponseError(body));
        }

        let Location(ref location) = *res.headers.get::<Location>().unwrap();
        let parts: Vec<&str> = location.split('/').collect();

        Ok(Path {
            collection: collection,
            key: parts[3].to_string(),
            ref_: Some(parts[5].to_string())
        })
    }
}

pub struct UpdateKeyValue<'a> {
    client: &'a mut Client,
    collection: String,
    key: String,
    url: String,
    data: Option<String>,
    ref_: Option<String>,
    if_absent: Option<bool>
}

impl<'a> UpdateKeyValue<'a> {

    pub fn new(client: &'a mut Client, collection: &str, key: &str)
               -> UpdateKeyValue<'a> {
        UpdateKeyValue {
            client: client,
            collection: collection.to_string(),
            key: key.to_string(),
            url: format!("{}/{}", collection.to_string(), key.to_string()),
            data: None,
            ref_: None,
            if_absent: None
        }
    }

    pub fn data<'b,
                T: Encodable<Encoder<'b>, IoError>>(
                    mut self,
                    data: &T) -> UpdateKeyValue<'a> {
        self.data = Some(json::encode(&data));
        self
    }

    pub fn if_match(mut self, ref_: &str) -> UpdateKeyValue<'a> {
        self.ref_ = Some(ref_.to_string());
        self
    }

    pub fn if_absent(mut self) -> UpdateKeyValue<'a> {
        self.if_absent = Some(true);
        self
    }

    pub fn exec(self) -> PathResult {
        let UpdateKeyValue {
          client, collection, key, url, data, ref_, if_absent
        } = self;

        let mut client = client.trailing(url.as_slice())
                               .body(data.unwrap().as_slice())
                               .method(Put);

        if ref_.is_some() {
            client.header("If-Match", ref_.unwrap().as_slice());
        }

        if if_absent.is_some() {
            client.header("If-None-Match", "*");
        }

        let mut res = try!(client.exec());
        let body = try!(res.read_to_string());

        if (res.status as i32) != 201 {
            return Err(ResponseError(body));
        }

        let Location(ref location) = *res.headers.get::<Location>().unwrap();
        let parts: Vec<&str> = location.split('/').collect();

        Ok(Path {
            collection: collection,
            key: key,
            ref_: Some(parts[5].to_string())
        })
    }
}

pub struct DeleteKeyValue<'a> {
    client: &'a mut Client,
    ref_: Option<String>,
    url: String
}

impl<'a> DeleteKeyValue<'a> {

    pub fn new(client: &'a mut Client, collection: &str, key: &str)
               -> DeleteKeyValue<'a> {
        DeleteKeyValue {
            client: client,
            ref_: None,
            url: format!("{}/{}", collection, key)
        }
    }

    pub fn if_match(mut self, ref_: &str) -> DeleteKeyValue<'a> {
        self.ref_ = Some(ref_.to_string());
        self
    }

    pub fn purge(mut self) -> DeleteKeyValue<'a> {
        self.client.query("purge", "true");
        self
    }

    pub fn exec(self) -> Result<bool, OrchestrateError> {
        let DeleteKeyValue { client, ref_, url } = self;
        let mut client = client.trailing(url.as_slice()).method(Delete);

        if ref_.is_some() {
            client.header("If-Match", ref_.unwrap().as_slice());
        }

        let mut res = try!(client.exec());

        if (res.status as i32) != 204 {
            return Err(ResponseError(try!(res.read_to_string())));
        }

        Ok(true)
    }
}

pub struct ListReader<'a> {
    collection: String,
    client: &'a mut Client
}

impl<'a> ListReader<'a> {
    pub fn new<'a>(client: &'a mut Client, collection: &str) -> ListReader<'a> {
        ListReader {
            collection: collection.to_string(),
            client: client
        }
    }

    pub fn limit(mut self, limit: int) -> ListReader<'a> {
        self.client.query("limit", limit.to_string().as_slice());
        self
    }

    pub fn start_key(mut self, start_key: &str) -> ListReader<'a> {
        self.client.query("startKey", start_key);
        self
    }

    pub fn after_key(mut self, after_key: &str) -> ListReader<'a> {
        self.client.query("afterKey", after_key);
        self
    }

    pub fn before_key(mut self, before_key: &str) -> ListReader<'a> {
        self.client.query("beforeKey", before_key);
        self
    }

    pub fn end_key(mut self, end_key: &str) -> ListReader<'a> {
        self.client.query("endKey", end_key);
        self
    }

    pub fn exec<T: RepresentsJSON>(self) -> KeyValueResults<T> {
        let ListReader { client, collection } = self;
        let mut res = try!(client.trailing(collection.as_slice())
                                 .method(Get).exec());
        let body = try!(res.read_to_string());

        if (res.status as i32) != 200 {
            return Err(ResponseError(body));
        }

        Ok(try!(json::decode::<KVResults<T>>(body.as_slice())))
    }
}

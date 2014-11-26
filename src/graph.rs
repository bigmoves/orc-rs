use client::Client;
use path::Path;
use error::OrchestrateError;
use error::OrchestrateError::RequestError;
use RepresentsJSON;
use serialize::json;
use hyper::method::{Get, Put, Delete};

#[deriving(Decodable, Encodable, Show)]
pub struct GraphResults<T> {
    pub count: u64,
    pub results: Vec<GraphResult<T>>,
}

#[deriving(Decodable, Encodable, Show)]
pub struct GraphResult<T> {
    pub path: Path,
    pub value: T
}

pub struct GetRelations<'a> {
    client: &'a mut Client,
    url: String
}

impl<'a> GetRelations<'a> {
    pub fn new(client: &'a mut Client, collection: &str, key: &str,
               hops: Vec<&str>) -> GetRelations<'a> {
        let relations_path = hops.connect("/");
        GetRelations {
            client: client,
            url: format!("{}/{}/relations/{}", collection, key, relations_path)
        }
    }

    pub fn limit(mut self, limit: int) -> GetRelations<'a> {
        self.client.query("limit", limit.to_string().as_slice());
        self
    }

    pub fn offset(mut self, offset: int) -> GetRelations<'a> {
        self.client.query("offset", offset.to_string().as_slice());
        self
    }

    pub fn exec<T: RepresentsJSON>(self)
                -> Result<GraphResults<T>, OrchestrateError> {
        let mut res = try!(self.client.trailing(self.url.as_slice())
                                      .method(Get).exec());
        let body = try!(res.read_to_string());

        if (res.status as i32) != 200 {
            return Err(RequestError(body));
        }

        Ok(try!(json::decode::<GraphResults<T>>(body.as_slice())))
    }
}

pub struct PutRelation<'a> {
    client: &'a mut Client,
    url: String
}

impl<'a> PutRelation<'a> {
    pub fn new(client: &'a mut Client, collection: &str, key: &str, kind: &str,
               to_collection: &str, to_key: &str) -> PutRelation<'a> {
        PutRelation {
            client: client,
            url: format!("{}/{}/relation/{}/{}/{}", collection, key, kind,
                         to_collection, to_key)
        }
    }

    pub fn exec(self) -> Result<bool, OrchestrateError> {
        let mut res = try!(self.client.trailing(self.url.as_slice())
                                      .method(Put).exec());

        if (res.status as i32) != 204 {
            return Err(RequestError(try!(res.read_to_string())));
        }

        Ok(true)
    }
}

pub struct DeleteRelation<'a> {
    client: &'a mut Client,
    url: String
}

impl<'a> DeleteRelation<'a> {
    pub fn new(client: &'a mut Client, collection: &str, key: &str, kind: &str,
               to_collection: &str, to_key: &str) -> DeleteRelation<'a> {
        DeleteRelation {
            client: client,
            url: format!("{}/{}/relation/{}/{}/{}", collection, key, kind,
                         to_collection, to_key)
        }
    }

    pub fn exec(self) -> Result<bool, OrchestrateError> {
        let mut res = try!(self.client.trailing(self.url.as_slice())
                                      .method(Delete).exec());

        if (res.status as i32) != 204 {
            return Err(RequestError(try!(res.read_to_string())));
        }

        Ok(true)
    }
}

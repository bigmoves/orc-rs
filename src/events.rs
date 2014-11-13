use client::Client;
use error::Error;
use url::form_urlencoded::serialize_owned as serialize;
use hyper::method::Get;
use serialize::json;
use RepresentsJSON;

#[deriving(Decodable, Encodable, Show)]
pub struct EventResults<T> {
    count: u64,
    results: Vec<EventResult<T>>,
}

#[deriving(Decodable, Encodable, Show)]
pub struct EventResult<T> {
    ordinal: u64,
    timestamp: u64,
    value: T
}

pub struct EventReader<'a, 'b> {
    collection: Option<String>,
    key: Option<String>,
    kind: Option<String>,
    time: Option<u64>,
    start: Option<u64>,
    end: Option<u64>,
    limit: Option<int>,
    ordinal: Option<u64>,
    client: &'a mut Client
}

impl<'a, 'b> EventReader<'a, 'b> {
    pub fn new(client: &'a mut Client) -> EventReader<'a, 'b> {
        EventReader {
            collection: None,
            key: None,
            kind: None,
            time: None,
            start: None,
            end: None,
            limit: None,
            ordinal: None,
            client: client
        }
    }

    pub fn from(mut self, collection: &str, key: &str) -> EventReader<'a, 'b> {
        self.collection = Some(collection.to_string());
        self.key = Some(key.to_string());
        self
    }

    pub fn kind(mut self, kind: &str) -> EventReader<'a, 'b> {
        self.kind = Some(kind.to_string());
        self
    }

    pub fn time(mut self, time: u64) -> EventReader<'a, 'b> {
        self.time = Some(time);
        self
    }

    pub fn start(mut self, start: u64) -> EventReader<'a, 'b> {
        self.start = Some(start);
        self
    }

    pub fn end(mut self, end: u64) -> EventReader<'a, 'b> {
        self.end = Some(end);
        self
    }

    pub fn limit(mut self, limit: int) -> EventReader<'a, 'b> {
        self.limit = Some(limit);
        self
    }

    pub fn ordinal(mut self, ordinal: u64) -> EventReader<'a, 'b> {
        self.ordinal = Some(ordinal);
        self
    }

    pub fn list<T: RepresentsJSON>(self) -> Result<EventResults<T>, Error> {
        let EventReader {
            collection,
            key,
            kind,
            start,
            end,
            limit,
            client,
            ..
        } = self;

        let mut query_params: Vec<(String, String)> = Vec::new();

        match start {
            Some(s) => query_params.push(("start".to_string(), s.to_string())),
            None => {}
        }

        match end {
            Some(e) => query_params.push(("end".to_string(), e.to_string())),
            None => {}
        }

        match limit {
            Some(l) => query_params.push(("limit".to_string(), l.to_string())),
            None => {}
        }

        let encoded_params = serialize(query_params.as_slice());
        let trailing_uri = format!("{}/{}/events/{}?{}",
                                   collection.unwrap().as_slice(),
                                   key.unwrap().as_slice(),
                                   kind.unwrap().as_slice(),
                                   encoded_params.as_slice());

        let res = client.request(Get, trailing_uri.as_slice(),
                                     None, None).unwrap();

        if res.code != 200 {
            return Err(Error::new(res.body.as_slice()));
        }

        Ok(json::decode::<EventResults<T>>(res.body.as_slice()).unwrap())
    }

    pub fn get<T: RepresentsJSON>(self) -> Result<EventResult<T>, Error> {
        let EventReader {
            collection,
            key,
            kind,
            time,
            ordinal,
            client,
            ..
        } = self;

        let trailing_uri = format!("{}/{}/events/{}/{}/{}",
                                   collection.unwrap().as_slice(),
                                   key.unwrap().as_slice(),
                                   kind.unwrap().as_slice(),
                                   time.unwrap(), ordinal.unwrap());
        let res = client.request(Get, trailing_uri.as_slice(),
                                     None, None).unwrap();

        if res.code != 200 {
            return Err(Error::new(res.body.as_slice()));
        }

        Ok(json::decode::<EventResult<T>>(res.body.as_slice()).unwrap())
    }
}


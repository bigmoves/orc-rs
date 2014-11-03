use url;

use std::str;
use std::str::StrSlice;
use std::io::IoError;

use client::{Client, Headers};
use path::Path;
use error::Error;

use std::collections::HashMap;
use serialize::json::Encoder;
use serialize::json;
use serialize::{Encodable};

#[deriving(Decodable, Encodable)]
pub struct KVResults {
    pub count: int,
    pub results: Vec<KVResult>,
    pub next: Option<String>
}

impl KVResults {
    fn has_next(&self) -> bool {
        self.next.is_none()
    }
}

#[deriving(Decodable, Encodable)]
pub struct KVResult {
    pub path: Path,
    pub value: HashMap<String, String>
}

pub trait KV {

    fn get(&mut self, collection: &str, key: &str) -> Result<KVResult, Error>;

    fn get_path(&mut self, path: Path) -> Result<KVResult, Error>;

    fn post<'a, T: Encodable<Encoder<'a>, IoError>>(&mut self, collection: &str, value: &T) -> Result<Path, Error>;

    fn post_raw(&mut self, collection: &str, value: &str) -> Result<Path, Error>;

    fn exec_post(&mut self, path: Path, value: &str) -> Result<Path, Error>;

    fn put<'a, T: Encodable<Encoder<'a>, IoError>>(&mut self, collection: &str, key: &str, value: &T) -> Result<Path, Error>;

    fn put_raw(&mut self, collection: &str, key: &str, value: &str) -> Result<Path, Error>;

    fn put_if_unmodified<'a, T: Encodable<Encoder<'a>, IoError>>(&mut self, path: Path, value: &T) -> Result<Path, Error>;

    fn put_if_unmodified_raw(&mut self, path: Path, value: &str) -> Result<Path, Error>;

    fn exec_put(&mut self, mut path: Path, headers: Option<Headers>, value: &str) -> Result<Path, Error>;

    fn list(&mut self, collection: &str, limit: int) -> Result<KVResults, Error>;

    fn list_after(&mut self, collection: &str, after: &str, limit: int) -> Result<KVResults, Error>;

    fn list_start(&mut self, collection: &str, start: &str, limit: int) -> Result<KVResults, Error>;

    fn list_range(&mut self, collection: &str, start: &str, end: &str, limit: int) -> Result<KVResults, Error>;

    fn get_list(&mut self, collection: &str) -> Result<KVResults, Error>;

    fn delete(&mut self, collection: &str, key: &str) -> Result<(), Error>;

    fn exec_delete(&mut self, trailing_uri: &str) -> Result<(), Error>;

}

impl KV for Client {
    fn get(&mut self, collection: &str, key: &str) -> Result<KVResult, Error> {
        self.get_path(Path {
            collection: collection.to_string(),
            key: Some(key.to_string()),
            ref_: None
        })
    }

    fn get_path(&mut self, mut path: Path) -> Result<KVResult, Error> {
        let res = self.get_request(format!("{}/{}", path.collection, path.key.clone().unwrap()).as_slice());

        if res.get_code() != 200 {
            return Err(Error::new(res));
        }

        let value = str::from_utf8(res.get_body());

        if path.ref_.is_none() {
            let content_location = res.get_header("content-location")[0].as_slice();
            let parts: Vec<&str> = content_location.split('/').collect();
            if parts.len() >= 6 {
                path.ref_ = Some(parts[5].to_string());
            }
        }

        Ok(KVResult { path: path, value: json::decode(value.unwrap()).unwrap() })
    }

    fn post<'a, T: Encodable<Encoder<'a>, IoError>>(&mut self, collection: &str, value: &T) -> Result<Path, Error> {
        let json_str = json::encode(&value);
        self.post_raw(collection, json_str.as_slice())
    }

    fn post_raw(&mut self, collection: &str, value: &str) -> Result<Path, Error> {
        self.exec_post(Path {
            collection: collection.to_string(),
            key: None,
            ref_: None
        }, value)
    }

    fn exec_post(&mut self, mut path: Path, value: &str) -> Result<Path, Error> {
        let res = self.post_request(path.collection.as_slice(), value);

        if res.get_code() != 201 {
            return Err(Error::new(res));
        }

        let location = res.get_header("location")[0].as_slice();
        let parts: Vec<&str> = location.split('/').collect();
        if parts.len() >= 6 {
            path.key = Some(parts[3].to_string());
            path.ref_ = Some(parts[5].to_string());
        }

        Ok(Path {
            collection: path.collection,
            key: path.key,
            ref_: path.ref_
        })
    }

    fn put<'a, T: Encodable<Encoder<'a>, IoError>>(&mut self, collection: &str, key: &str, value: &T) -> Result<Path, Error> {
        let json_str = json::encode(&value);
        self.put_raw(collection, key, json_str.as_slice())
    }

    fn put_raw(&mut self, collection: &str, key: &str, value: &str) -> Result<Path, Error> {
        self.exec_put(Path {
            collection: collection.to_string(),
            key: Some(key.to_string()),
            ref_: None
        }, None, value)
    }

    fn put_if_unmodified<'a, T: Encodable<Encoder<'a>, IoError>>(&mut self, path: Path, value: &T) -> Result<Path, Error> {
        let json_str = json::encode(&value);
        self.put_if_unmodified_raw(path, json_str.as_slice())
    }

    fn put_if_unmodified_raw(&mut self, path: Path, value: &str) -> Result<Path, Error> {
        let mut headers = HashMap::new();
        headers.insert("If-Match".to_string(), format!("{}", path.ref_));

        self.exec_put(path, Some(headers), value)
    }

    fn exec_put(&mut self, mut path: Path, headers: Option<Headers>, value: &str) -> Result<Path, Error> {
        let res = self.put_request(format!("{}/{}", path.collection, path.key).as_slice(), headers, value);

        if res.get_code() != 201 {
            return Err(Error::new(res));
        }

        if path.ref_.is_none() {
            let location = res.get_header("location")[0].as_slice();
            let parts: Vec<&str> = location.split('/').collect();
            if parts.len() >= 6 {
                path.ref_ = Some(parts[5].to_string());
            }
            // add error if no ref in location header
        }

        Ok(Path {
            collection: path.collection,
            key: path.key,
            ref_: path.ref_
        })
    }

    fn list(&mut self, collection: &str, limit: int) -> Result<KVResults, Error> {
        let query_params = [
            ("limit".to_string(), limit.to_string())
        ];
        let encoded_params = url::form_urlencoded::serialize_owned(query_params.as_slice());
        let trailing_uri = format!("{}?{}", collection, encoded_params.as_slice());

        self.get_list(trailing_uri.as_slice())
    }

    fn list_after(&mut self, collection: &str, after: &str, limit: int) -> Result<KVResults, Error> {
        let query_params = [
            ("limit".to_string(), limit.to_string()),
            ("afterKey".to_string(), after.to_string())
        ];
        let encoded_params = url::form_urlencoded::serialize_owned(query_params.as_slice());
        let trailing_uri = format!("{}?{}", collection, encoded_params.as_slice());

        self.get_list(trailing_uri.as_slice())
    }

    fn list_start(&mut self, collection: &str, start: &str, limit: int) -> Result<KVResults, Error> {
        let query_params = [
            ("limit".to_string(), limit.to_string()),
            ("startKey".to_string(), start.to_string())
        ];
        let encoded_params = url::form_urlencoded::serialize_owned(query_params.as_slice());
        let trailing_uri = format!("{}?{}", collection, encoded_params.as_slice());

        self.get_list(trailing_uri.as_slice())
    }

    fn list_range(&mut self, collection: &str, start: &str, end: &str, limit: int) -> Result<KVResults, Error> {
        let query_params = [
            ("limit".to_string(), limit.to_string()),
            ("startKey".to_string(), start.to_string()),
            ("endKey".to_string(), end.to_string())
        ];
        let encoded_params = url::form_urlencoded::serialize_owned(query_params.as_slice());
        let trailing_uri = format!("{}?{}", collection, encoded_params.as_slice());

        self.get_list(trailing_uri.as_slice())
    }

    fn get_list(&mut self, trailing_uri: &str) -> Result<KVResults, Error> {
        let res = self.get_request(trailing_uri);

        if res.get_code() != 200 {
            return Err(Error::new(res));
        }

        let results = str::from_utf8(res.get_body());

        Ok(json::decode(results.unwrap()).unwrap())
    }

    fn delete(&mut self, collection: &str, key: &str) -> Result<(), Error> {
        self.exec_delete(format!("{}/{}", collection, key).as_slice())
    }

    fn exec_delete(&mut self, trailing_uri: &str) -> Result<(), Error> {
        let res = self.delete_request(trailing_uri);

        if res.get_code() != 204 {
            return Err(Error::new(res));
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use client::Client;
    use key_value::KV;

    #[test]
    fn test_kv() {
        let mut client = Client::new(env!("ORC_API_KEY"));
        let mut user = HashMap::new();
        user.insert("name", "chad");
        user.insert("email", "chadtmiller15@gmail.com");
        let path = client.post("users", &user).unwrap();
        assert!(path.collection == "users".to_string());
    }
}

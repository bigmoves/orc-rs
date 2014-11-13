use serialize::json;

#[deriving(Encodable, Decodable, Show)]
pub struct Error {
    pub message: String
}

impl Error {
    pub fn new(res: &str) -> Error {
        json::decode(res).unwrap()
    }
}


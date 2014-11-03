use curl::http;
use serialize::json;
use std::str;

#[deriving(Decodable)]
pub struct ErrMessage {
    message: String
}

#[deriving(Encodable, Show)]
pub struct Error {
    pub status: uint,
    pub message: String
}

impl Error {
    pub fn new(res: http::Response) -> Error {
        let message: ErrMessage = json::decode(str::from_utf8(res.get_body()).unwrap()).unwrap();

        Error {
            status: res.get_code(),
            message: message.message
        }
    }
}


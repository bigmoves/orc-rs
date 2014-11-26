use serialize::json;
use hyper;
use std::io;
use std::error;

#[deriving(Show)]
pub enum OrchestrateError {
    JsonError(json::DecoderError),
    HttpError(hyper::HttpError),
    RequestError(String),
    IoError(io::IoError)
}

impl error::Error for OrchestrateError {
    fn description(&self) -> &str {
        match *self {
            OrchestrateError::JsonError(_) => "failed to decode json",
            OrchestrateError::HttpError(ref err) => err.description(),
            OrchestrateError::RequestError(ref err) => err.as_slice(),
            OrchestrateError::IoError(ref err) => err.description()
        }
    }

    fn detail(&self) -> Option<String> {
        match *self {
            _ => None
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            _ => None
        }
    }
}

impl error::FromError<json::DecoderError> for OrchestrateError {
    fn from_error(err: json::DecoderError) -> OrchestrateError {
        OrchestrateError::JsonError(err)
    }
}

impl error::FromError<hyper::HttpError> for OrchestrateError {
    fn from_error(err: hyper::HttpError) -> OrchestrateError {
        OrchestrateError::HttpError(err)
    }
}

impl error::FromError<io::IoError> for OrchestrateError {
    fn from_error(err: io::IoError) -> OrchestrateError {
        OrchestrateError::IoError(err)
    }
}

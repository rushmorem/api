// Copyright 2015-2016 Intecture Developers. See the COPYRIGHT file at the
// top-level directory of this distribution and at
// https://intecture.io/COPYRIGHT.
//
// Licensed under the Mozilla Public License 2.0 <LICENSE or
// https://www.tldrlegal.com/l/mpl-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

#[cfg(feature = "remote-run")]
use czmq;
use regex;
use rustc_serialize::json;
use std::{convert, error, fmt, io, num, str, string};
#[cfg(feature = "remote-run")]
use zfilexfer;

#[derive(Debug)]
pub enum Error {
    /// An error string returned from the host's Intecture Agent
    Agent(String),
    #[cfg(feature = "remote-run")]
    /// An error string returned from the host's Intecture Auth
    Auth(String),
    #[cfg(feature = "remote-run")]
    /// CZMQ error
    Czmq(czmq::Error),
    /// JSON decoder error
    JsonDecoder(json::DecoderError),
    #[cfg(feature = "remote-run")]
    /// Message frames missing in the response from host's Intecture Agent
    Frame(MissingFrame),
    /// Generic error string
    Generic(String),
    #[cfg(feature = "remote-run")]
    /// Cannot run command on disconnected host
    HostDisconnected,
    #[cfg(feature = "remote-run")]
    /// Invalid response from host
    HostResponse,
    /// IO error
    Io(io::Error),
    /// Cast str as float
    ParseFloat(num::ParseFloatError),
    /// Cast str as int
    ParseInt(num::ParseIntError),
    /// Regex error
    Regex(regex::Error),
    /// Cast str
    StrFromUtf8(str::Utf8Error),
    /// Cast String
    StringFromUtf8(string::FromUtf8Error),
    #[cfg(feature = "remote-run")]
    /// ZFileXfer error
    ZFileXfer(zfilexfer::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Agent(ref e) => write!(f, "Agent error: {}", e),
            #[cfg(feature = "remote-run")]
            Error::Auth(ref e) => write!(f, "Auth error: {}", e),
            #[cfg(feature = "remote-run")]
            Error::Czmq(ref e) => write!(f, "CZMQ error: {}", e),
            Error::JsonDecoder(ref e) => write!(f, "JSON decoder error: {}", e),
            #[cfg(feature = "remote-run")]
            Error::Frame(ref e) => write!(f, "Missing frame {} in message: {}", e.order, e.name),
            Error::Generic(ref e) => write!(f, "Error: {}", e),
            #[cfg(feature = "remote-run")]
            Error::HostDisconnected => write!(f, "Cannot run command while host is disconnected"),
            #[cfg(feature = "remote-run")]
            Error::HostResponse => write!(f, "Invalid response from host"),
            Error::Io(ref e) => write!(f, "IO error: {}", e),
            Error::ParseFloat(ref e) => write!(f, "Parse error: {}", e),
            Error::ParseInt(ref e) => write!(f, "Parse error: {}", e),
            Error::Regex(ref e) => write!(f, "Regex error: {}", e),
            Error::StrFromUtf8(ref e) => write!(f, "Convert from UTF8 slice to str error: {}", e),
            Error::StringFromUtf8(ref e) => write!(f, "Convert from UTF8 slice to String error: {}", e),
            #[cfg(feature = "remote-run")]
            Error::ZFileXfer(ref e) => write!(f, "ZFileXfer error: {}", e),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Agent(ref e) => e,
            #[cfg(feature = "remote-run")]
            Error::Auth(ref e) => e,
            #[cfg(feature = "remote-run")]
            Error::Czmq(ref e) => e.description(),
            Error::JsonDecoder(ref e) => e.description(),
            #[cfg(feature = "remote-run")]
            Error::Frame(_) => "The Agent's reply was missing a part ('frame') of the expected message",
            Error::Generic(ref e) => e,
            #[cfg(feature = "remote-run")]
            Error::HostDisconnected => "Cannot run command on disconnected host",
            #[cfg(feature = "remote-run")]
            Error::HostResponse => "Invalid response from host",
            Error::Io(ref e) => e.description(),
            Error::ParseFloat(ref e) => e.description(),
            Error::ParseInt(ref e) => e.description(),
            Error::Regex(ref e) => e.description(),
            Error::StrFromUtf8(ref e) => e.description(),
            Error::StringFromUtf8(ref e) => e.description(),
            #[cfg(feature = "remote-run")]
            Error::ZFileXfer(ref e) => e.description(),
        }
    }
}

#[cfg(feature = "remote-run")]
impl convert::From<czmq::Error> for Error {
    fn from(err: czmq::Error) -> Error {
        Error::Czmq(err)
    }
}

impl convert::From<json::DecoderError> for Error {
    fn from(err: json::DecoderError) -> Error {
        Error::JsonDecoder(err)
    }
}

#[cfg(feature = "remote-run")]
impl convert::From<MissingFrame> for Error {
    fn from(err: MissingFrame) -> Error {
        Error::Frame(err)
    }
}

impl convert::From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl convert::From<regex::Error> for Error {
    fn from(err: regex::Error) -> Error {
        Error::Regex(err)
    }
}

impl convert::From<str::Utf8Error> for Error {
	fn from(err: str::Utf8Error) -> Error {
		Error::StrFromUtf8(err)
	}
}

impl convert::From<string::FromUtf8Error> for Error {
	fn from(err: string::FromUtf8Error) -> Error {
		Error::StringFromUtf8(err)
	}
}

impl convert::From<num::ParseFloatError> for Error {
	fn from(err: num::ParseFloatError) -> Error {
		Error::ParseFloat(err)
	}
}

impl convert::From<num::ParseIntError> for Error {
	fn from(err: num::ParseIntError) -> Error {
		Error::ParseInt(err)
	}
}

#[cfg(feature = "remote-run")]
impl convert::From<zfilexfer::Error> for Error {
    fn from(err: zfilexfer::Error) -> Error {
        Error::ZFileXfer(err)
    }
}

#[cfg(feature = "remote-run")]
#[derive(Debug)]
pub struct MissingFrame {
    name: String,
    order: u8,
}

#[cfg(feature = "remote-run")]
impl MissingFrame {
    pub fn new(name: &str, order: u8) -> MissingFrame {
        MissingFrame {
            name: name.to_string(),
            order: order,
        }
    }
}

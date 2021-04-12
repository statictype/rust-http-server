use super::method::{Method, MethodError};
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
// &str not String is ok because we never want to change the request anyway, so why own it?
// but the buffer memory the slices point to might be freed, so we specify a lifetime '
// lifetimes are metadata for the compiler to understand that the request try_from returns
// has a relationship with the memory of the buffer = memory safety
pub struct Request<'buf> {
    path: &'buf str,
    // query string should be more than Option<&'buf str>
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
    // cannot use the request in the handler because all the fields are private
    // must implement getters
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn method(&self) -> &Method {
        &self.method
    }
    // ???
    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

// parse the request
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;
    // tries to make a request type from a byte slice
    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        // GET /search?name="abc"&sort=1 HTTP/1.1\r\n...headers...
        // we only support http 1 and don't care about headers atm
        let request = str::from_utf8(buf)?;
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        // convert the method string slice into method variant
        // parse() calls fromstr impl in method?
        let method: Method = method.parse()?;
        // if there is a "?" in the path string slice
        // everything after it goes to query string
        // path becomes everything before it
        let mut query_string = None;
        if let Some(i) = path.find("?") {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}
// returns (word, restofstring)
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // break request str by space and newline
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            // ususally not safe to add 1 to array indices, some chars can have more than one byte
            // it would crash, but in this case we know it's a space so it's cool
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

// all errors that we can think of while parsing the req
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
// make parseerror more idiomatic
// forces to meet some basic expectations like having the display and debug traits
impl Error for ParseError {}

// more codes should be implemented
use std::fmt::{Display, Formatter, Result as FmtResult};
#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // to cast something to a primitive type use the "as" keyword
        // can't cast references, must dereference
        // must implement copy trait
        write!(f, "{}", *self as u16)
    }
}

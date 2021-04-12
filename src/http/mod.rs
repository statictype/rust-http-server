// export types (structs) so that they are usable outside their modules
pub use method::Method;
pub use query_string::{QueryString, Value as QueryStringValue};
pub use request::ParseError;
pub use request::Request;
pub use response::Response;
pub use status_code::StatusCode;

// the compiler only knows about the modules specified here
pub mod method;
pub mod query_string;
pub mod request;
pub mod response;
pub mod status_code;

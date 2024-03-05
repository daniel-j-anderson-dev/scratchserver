mod request;
mod response;
mod url;
mod verb;

pub use self::{request::Request, response::Response, verb::Verb, url::Url};

use std::str::FromStr;

pub enum Version {
    Http0_9,
    Http1_0,
    Http1_1,
    Http2,
    Http3,
}
// impl FromStr for Version {
//     type Err = Box<dyn std::error::Error>;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
        
//     }
// }
mod method;
mod request;
mod response;
mod url;

pub use self::{method::Method, request::Request, response::Response, url::Url};

use color_eyre::{eyre::eyre, Report};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub enum Version {
    Http0_9,
    Http1_0,
    Http1_1,
    Http2,
    Http3,
}
impl FromStr for Version {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "HTTP/0.9" => Ok(Self::Http0_9),
            "HTTP/1.0" => Ok(Self::Http1_0),
            "HTTP/1.1" => Ok(Self::Http1_1),
            "HTTP/2" => Ok(Self::Http2),
            "HTTP/3" => Ok(Self::Http3),
            invalid => Err(eyre!("{} is not a valid HTTP Version", invalid)),
        };
    }
}
impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{}",
            match self {
                Self::Http0_9 => "HTTP/0.9",
                Self::Http1_0 => "HTTP/1.0",
                Self::Http1_1 => "HTTP/1.1",
                Self::Http2 => "HTTP/2",
                Self::Http3 => "HTTP/3",
            }
        );
    }
}

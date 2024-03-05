use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum Verb {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}
impl FromStr for Verb {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let verb = match s {
            "GET" => Self::Get,
            "HEAD" => Self::Head,
            "POST" => Self::Post,
            "PUT" => Self::Put,
            "DELETE" => Self::Delete,
            "CONNECT" => Self::Connect,
            "OPTIONS" => Self::Options,
            "TRACE" => Self::Trace,
            "PATCH" => Self::Patch,
            invalid_verb => Err(format!("{} is not a valid HTTP Verb", invalid_verb))?,
        };
        return Ok(verb);
    }
}
impl Display for Verb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{}",
            match self {
                Self::Get => "GET",
                Self::Head => "HEAD",
                Self::Post => "POST",
                Self::Put => "PUT",
                Self::Delete => "DELETE",
                Self::Connect => "CONNECT",
                Self::Options => "OPTIONS",
                Self::Trace => "TRACE",
                Self::Patch => "PATCH",
            }
        );
    }
}

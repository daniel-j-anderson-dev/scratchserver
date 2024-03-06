use color_eyre::{eyre::eyre, Report};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Default, Clone, Copy)]
pub enum Method {
    #[default]
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
impl FromStr for Method {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let verb = match s.to_lowercase().as_str() {
            "get" => Self::Get,
            "head" => Self::Head,
            "post" => Self::Post,
            "put" => Self::Put,
            "delete" => Self::Delete,
            "connect" => Self::Connect,
            "options" => Self::Options,
            "trace" => Self::Trace,
            "patch" => Self::Patch,
            invalid_verb => Err(eyre!("{} is not a valid HTTP Verb", invalid_verb))?,
        };
        return Ok(verb);
    }
}
impl Display for Method {
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

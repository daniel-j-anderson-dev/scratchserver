use crate::http::{url::Url, verb::Verb};

use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub struct Request {
    verb: Verb,
    url: Url,
}
impl FromStr for Request {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let verb = match s
            .split_whitespace()
            .filter_map(|word| word.parse::<Verb>().ok())
            .next()
        {
            Some(verb) => verb,
            None => Err("No http Verb found in request")?,
        };

        let url = match s
            .split_whitespace()
            .filter_map(|word| word.parse::<Url>().ok())
            .next()
        {
            Some(url) => url,
            None => Err("Url could not be parsed")?,
        };

        return Ok(Request { verb, url });
    }
}

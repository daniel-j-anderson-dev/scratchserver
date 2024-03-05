use crate::http::{verb::Verb};

use std::str::FromStr;

#[derive(Debug)]
pub struct Request {
    verb: Verb,
    path: String,
}
impl FromStr for Request {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();

        let verb = match words.next().and_then(|word| word.parse::<Verb>().ok()) {
            Some(verb) => verb,
            None => {
                Err("No http Verb found in request")?
            },
        };

        let path = match words.next() {
            Some(url) => url.to_string(),
            None => Err("Url could not be parsed")?,
        };

        return Ok(Request { verb, path });
    }
}

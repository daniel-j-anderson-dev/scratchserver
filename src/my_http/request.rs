use crate::my_http::{method::Method, url::path::Path, Version};

use color_eyre::{eyre::eyre, Report};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
    str::FromStr,
};

#[derive(Debug)]
pub struct Request {
    method: Method,
    path: String,
    version: Version,
    headers: HashMap<String, String>,
    body: String,
}
impl Request {
    pub const MIN_BYTES_READ: usize = 2;
    pub fn version(&self) -> Version {
        return self.version;
    }
    pub fn method(&self) -> Method {
        return self.method;
    }
    pub fn path(&self) -> &str {
        return &self.path;
    }
}
impl TryFrom<BufReader<TcpStream>> for Request {
    type Error = Report;
    /// Consume a [BufReader]<[TcpStream]> trying to read a line into a request
    fn try_from(mut reader: BufReader<TcpStream>) -> Result<Self, Self::Error> {
        let mut input = String::new();

        while reader.read_line(&mut input)? > Request::MIN_BYTES_READ {}

        return input.parse();
    }
}
impl FromStr for Request {
    type Err = Report;

    /// This function parses a raw request [String] into a [Request]
    fn from_str(raw_request: &str) -> Result<Self, Self::Err> {
        let mut lines = raw_request.lines();

        // parse the first line
        let mut first_line_words = lines
            .next()
            .ok_or(eyre!("in Request::from_str raw_request is empty"))?
            .split_whitespace();

        let method = first_line_words
            .next()
            .ok_or(eyre!("in Request::from_str first_line is missing method"))?
            .parse()
            .map_err(|parse_error| {
                eyre!(
                    "in Request::from_str could not parse method in first_line of raw_request\n{}",
                    parse_error
                )
            })?;

        let path = first_line_words
            .next()
            .ok_or(eyre!(
                "in Request::from_str first_line of raw_request is missing a path"
            ))?
            .to_string();

        let version = first_line_words
            .next()
            .ok_or(eyre!("in Request::from_str first_line of raw_request is missing a HTTP Version"))?
            .parse().map_err(|parse_error| {
                eyre!("in Request::from_str HTTP Version could not be parsed from first_line of raw_request\n{}", parse_error)
            })?;

        // parse headers
        let mut headers = HashMap::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            let colon_index = line
                .find(':')
                .ok_or(eyre!("{} is not a valid header", line))?;
            let (key, value) = line.split_at(colon_index);
            headers
                .entry(key.to_string())
                .or_insert(value.chars().skip(2).collect());
        }

        // body
        let body = lines.collect::<String>();

        return Ok(Request {
            method,
            path,
            version,
            headers,
            body,
        });
    }
}

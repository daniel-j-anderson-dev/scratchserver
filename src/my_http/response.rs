mod status;

use crate::{
    my_http::{request::Request, response::status::StatusCode, Version},
    SERVE_DIR,
};

use color_eyre::{owo_colors::OwoColorize, Report};
use flate2::{read::GzEncoder, Compression};
use std::{
    collections::HashMap, fmt::Display, fs::File, io::{BufReader, Read, Write}, path::{Path, PathBuf}
};

#[derive(Debug)]
pub struct Response {
    version: Version,
    headers: HashMap<String, String>,
    body: Vec<u8>,
    status: StatusCode,
}
impl Response {
    pub fn start_line(&self) -> String {
        return format!("{} {}", self.version, self.status);
    }
    pub fn headers(&self) -> String {
        let mut headers = String::new();
        for (key, value) in self.headers.iter() {
            headers.push_str(format!("{}: {}\r\n", key, value).as_str());
        }
        return headers;
    }
    /// returns the number of bytes in `body`
    pub fn body_length(&self) -> usize {
        return self.body.len();
    }
    ///  Encodes the body using gz spec
    fn gz_encoded_body(&self) -> Result<Vec<u8>, Report> {
        let mut encoded_body = Vec::with_capacity(self.body_length());

        let mut encoder = GzEncoder::new(
            self.body.as_slice(),
            Compression::default(),
        );

        encoder.read_to_end(&mut encoded_body)?;

        return Ok(encoded_body);
    }
    fn get_content_type(file_path: &Path) -> Option<&'static str> {
        return file_path
            .extension()
            .and_then(|extension| extension.to_str())
            .and_then(|extension| match extension
            {
                "html" => Some("text/html"),
                "ico" => Some("x-icon"),
                "jpg" | "jpeg" => Some("image/jpg"),
                "gif" => Some("image/gif"),
                "png" => Some("image/png"),
                _ => None,
            });
    }
}
impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\r\n", self.start_line())?;

        write!(f, "{}\r\n", self.headers())?;

        match String::from_utf8(self.body.clone()) {
            Ok(body_string) => write!(f, "\r\n{}\r\n\r\n", body_string)?,
            Err(_) => match self.gz_encoded_body() {
                Ok(encoded_body) => write!(f, "\r\n{:?}\r\n\r\n", encoded_body)?,
                Err(_) => return Err(std::fmt::Error),
            }
        }

        return Ok(());
    }
}
impl TryFrom<Request> for Response {
    type Error = Report;
    fn try_from(value: Request) -> Result<Self, Self::Error> {
        let path = if value.path() == "/" {
            "/index.html"
        } else {
            value.path()
        };

        let file_path = PathBuf::from(format!("{}{}", SERVE_DIR, path));

        let mut file_data = Vec::new();
        let bytes_read =
            File::open(&file_path).and_then(|mut file| file.read_to_end(&mut file_data));

        let (body, status) = match bytes_read {
            Ok(_) => (file_data, StatusCode::OK),
            Err(io_error) => {
                println!(
                    "in Response::TryFrom<Request> could not read {} because {}\nsetting body to empty",
                    file_path.display(),
                    io_error
                );
                (Vec::new(), StatusCode::NOT_FOUND)
            }
        };

        let mut headers = HashMap::new();
        headers
            .entry("Content-Length".to_string())
            .or_insert(body.len().to_string());
        if let Some(content_type) = Response::get_content_type(&file_path) {
            headers
                .entry("Content-Type".to_string())
                .or_insert(content_type.to_owned());

            if content_type != "text/html" {
                headers
                    .entry("Content-Encoding".to_string())
                    .or_insert("gzip".to_string());
                headers
                    .entry("Accept-Ranges".to_string())
                    .or_insert("bytes".to_string());
            }
        }

        return Ok(Response {
            body,
            status,
            headers,
            version: value.version(),
        });
    }
}

#[cfg(test)]
mod test {

    use crate::{handle_connection, split_stream};

    use super::*;
    use std::{
        net::{TcpListener, TcpStream},
        thread,
    };

    #[test]
    fn test_gz_encoded_body() {
        const LOCALHOST: &str = "localhost:8000";

        let server_task = thread::spawn(|| {
            let stream = TcpListener::bind(LOCALHOST).and_then(|listener| {
                listener
                    .incoming()
                    .next()
                    .expect("incoming().next() is never none")
            });
            let split_stream = split_stream(stream).unwrap();
            handle_connection(split_stream).unwrap();
        });

        let client_task = thread::spawn(|| {
            let mut stream = TcpStream::connect(LOCALHOST).unwrap();
            stream
                .write(b"GET /assets/duck_hat.jpg HTTP/1.1\r\n")
                .unwrap();
        });

        server_task.join().unwrap();
        client_task.join().unwrap();
    }
}

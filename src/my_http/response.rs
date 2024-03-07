mod status;

use crate::{
    my_http::{body::Body, request::Request, response::status::StatusCode, Version},
    SERVE_DIR,
};

use color_eyre::{owo_colors::OwoColorize, Report};
use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Response {
    version: Version,
    headers: HashMap<String, String>,
    body: Body,
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
    pub fn body(&self) -> &Body {
        return &self.body;
    }
    /// Writes `self` in proper http format to `writer`
    pub fn send(&self, writer: &mut dyn Write) -> Result<(), std::io::Error> {
        writer.write(self.start_line().as_bytes())?;
        writer.write(b"\r\n")?;
        writer.write(self.headers().as_bytes())?;
        writer.write(b"\r\n")?;
        writer.write(b"\r\n")?;
        writer.write(self.body.as_ref())?;
        writer.write(b"\r\n")?;

        return Ok(());
    }

    fn get_content_type(file_path: &Path) -> Option<&'static str> {
        return file_path
            .extension()
            .and_then(|extension| extension.to_str())
            .and_then(|extension| match extension {
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
        write!(f, "{}\r\n", self.start_line().on_blue())?;
        write!(f, "{}\r\n", self.headers().on_blue())?;
        write!(
            f,
            "\r\n{}\r\n",
            if self.body().len() > 1000 {
                "body removed for brevity".into()
            } else {
                String::from_utf8_lossy(self.body.as_ref())
            }
            .on_blue()
        )?;

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
            Ok(_) => (Body::from(file_data), StatusCode::OK),
            Err(io_error) => {
                println!(
                    "in Response::TryFrom<Request> could not read {} because {}\nsetting body to empty",
                    file_path.display(),
                    io_error
                );
                (Body::from(Vec::new()), StatusCode::NOT_FOUND)
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
                // headers
                // .entry("Accept-Ranges".to_string())
                // .or_insert("bytes".to_string());
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
        io::Write,
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

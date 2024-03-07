use std::{borrow::Cow, io::Write, u8};

use flate2::{write::GzEncoder, Compression};

#[derive(Debug, Clone)]
pub enum Body {
    String(String),
    Data(Vec<u8>),
}
impl Body {
    pub fn len(&self) -> usize {
        return match self {
            Body::String(s) => s.len(),
            Body::Data(s) => s.len(),
        };
    }
    ///  Returns the encoded data as a [Cow]<[str]>
    pub fn encode(&self) -> Cow<[u8]> {
        let mut encoder = GzEncoder::new(Vec::with_capacity(self.len()), Compression::default());

        return match encoder
            .write_all(self.as_ref())
            .and_then(|_| encoder.finish())
        {
            Ok(encoded_body) => Cow::from(encoded_body),
            Err(_) => Cow::from(self.as_ref()),
        };
    }
    pub fn as_cow_u8(&self) -> Cow<[u8]> {
        return match self {
            Body::String(s) => Cow::from(s.as_bytes()),
            Body::Data(d) => Cow::from(d),
        };
    }
    pub fn as_cow_str(&self) -> Cow<str> {
        return match self {
            Body::String(s) => Cow::from(s.as_str()),
            Body::Data(d) => String::from_utf8_lossy(d.as_ref()),
        };
    }
}
impl From<Vec<u8>> for Body {
    fn from(value: Vec<u8>) -> Self {
        return if std::str::from_utf8(&value).is_ok() {
            Body::String(String::from_utf8(value).expect("std::str::from_utf8 returned Ok"))
        } else {
            Body::Data(value)
        };
    }
}
impl From<&[u8]> for Body {
    /// Clone a [AsRef<u8>] into either a [String] or a [Vec]
    fn from(data: &[u8]) -> Self {
        return match std::str::from_utf8(data) {
            Ok(str) => Body::String(str.to_owned()),
            Err(_) => Body::Data(data.to_owned()),
        };
    }
}

impl AsRef<[u8]> for Body {
    fn as_ref(&self) -> &[u8] {
        return match self {
            Body::String(s) => s.as_bytes(),
            Body::Data(d) => d.as_ref(),
        }
    }
}

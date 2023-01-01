use std::char::ParseCharError;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::ops::Deref;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub struct SimpleError {
    msg: String,
    source: Option<Box<dyn Error>>,
}

impl SimpleError {
    pub fn new(msg: String) -> Self {
        Self { msg, source: None }
    }
}

impl PartialEq for SimpleError {
    fn eq(&self, other: &Self) -> bool {
        self.msg == other.msg
    }
}

impl Display for SimpleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for SimpleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|t| t.deref())
    }
}

trait ErrorWrapper : Error + 'static {}

impl ErrorWrapper for ParseIntError {}

impl ErrorWrapper for ParseCharError {}

impl ErrorWrapper for FromUtf8Error {}

impl<T: ErrorWrapper> From<T> for SimpleError {
    fn from(t: T) -> Self {
        Self { msg: t.to_string(), source: Some(Box::new(t)) }
    }
}
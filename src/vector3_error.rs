use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Write};

pub enum Vector3ErrorKind {
    Deserialization(Vector3DeserializationErrorKind),
}

pub enum Vector3DeserializationErrorKind {
    InsufficientNumberOfValues(usize),
}

pub struct Vector3Error {
    kind: Vector3ErrorKind,
}

impl Vector3Error {
    pub fn new(kind: Vector3ErrorKind) -> Self {
        Self { kind }
    }
}

impl Debug for Vector3Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self.kind {
                Vector3ErrorKind::Deserialization(kind) => match kind {
                    Vector3DeserializationErrorKind::InsufficientNumberOfValues(n) => {
                        format!("Expected 3 values, got {}", n)
                    }
                },
            }
        )
    }
}

impl Display for Vector3Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self.kind {
                Vector3ErrorKind::Deserialization(kind) => match kind {
                    Vector3DeserializationErrorKind::InsufficientNumberOfValues(n) => {
                        format!("Expected 3 values, got {}", n)
                    }
                },
            }
        )
    }
}

impl Error for Vector3Error {}

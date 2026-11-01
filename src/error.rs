use std::{error::Error, fmt::Display};

use crate::status_code::StatusCode;

/**
 * Parse Error
 */

#[repr(u8)]
#[derive(Debug)]
pub enum ParseErrorKind {
    InvalidHeaderSize,
    MagicMismatch,
    InvalidVersion,
    UnknownMethod,
    UnknownContentType,
    BodyIsTooLarge,
    BodyIsTooBigForMethod,
    PathLengthIsTooLong,
    FailedToReadPathAndBody,
    InvalidUTF8InPath,
    InvalidUTF8InBody,
}

#[derive(Debug)]
pub struct ParseError {
    pub request_id: u32,
    pub status_code: StatusCode,
    pub kind: ParseErrorKind,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {:?}", self.request_id, self.kind)
    }
}
impl Error for ParseError {}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StatusCode {
    Success = 0,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
    InvalidHeader = 999,
}

impl StatusCode {
    /**
     * @brief Converts a two-byte array into a StatusCode.
     *
     * @param byte The two-byte array representing the status code.
     * @return Ok(StatusCode) if conversion succeeds, otherwise
     *         Err(&'static str) describing the error.
     */
    pub fn from_bytes(byte: [u8; 2]) -> Result<StatusCode, &'static str> {
        match u16::from_be_bytes(byte) {
            0 => Ok(StatusCode::Success),
            404 => Ok(StatusCode::NotFound),
            _ => Err("Unknown status code"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_status_code() -> Result<(), &'static str> {
        assert_eq!(StatusCode::from_bytes([0, 0])?, StatusCode::Success);
        Ok(())
    }
}

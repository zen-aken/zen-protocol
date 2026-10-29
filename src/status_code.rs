#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StatusCode {
    Success = 0,
    UnknownError = 999,
}

impl StatusCode {
    pub fn from_bytes(byte: [u8; 2]) -> Result<StatusCode, &'static str> {
        match u16::from_be_bytes(byte) {
            0 => Ok(StatusCode::Success),
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

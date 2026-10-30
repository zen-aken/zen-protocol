#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Methods {
    GET = 0x00,
    POST = 0x01,
    UPDATE = 0x02,
    DELETE = 0x03,
}

impl Methods {
    /**
     * @brief Parses a byte into a Methods variant.
     *
     * @param byte The byte representing the HTTP-like request method.
     * @return Ok(Methods) if the byte matches a valid method,
     *         otherwise Err(&'static str) describing the error.
     */
    pub fn parse(byte: u8) -> Result<Methods, &'static str> {
        match byte {
            0 => Ok(Methods::GET),
            1 => Ok(Methods::POST),
            2 => Ok(Methods::UPDATE),
            3 => Ok(Methods::DELETE),
            _ => Err("Unknown method"),
        }
    }

    /**
     * @brief Returns the maximum allowed body size for this method.
     *
     * @return Maximum body size in bytes.
     */
    pub fn max_body_size(&self) -> usize {
        match self {
            Methods::GET | Methods::DELETE => 0,
            Methods::POST | Methods::UPDATE => 16 * 1024 * 1024,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_method() -> Result<(), &'static str> {
        assert_eq!(Methods::parse(0).unwrap(), Methods::GET);
        assert_eq!(Methods::parse(1).unwrap(), Methods::POST);
        assert_eq!(Methods::parse(2).unwrap(), Methods::UPDATE);
        assert_eq!(Methods::parse(3).unwrap(), Methods::DELETE);
        Ok(())
    }
}

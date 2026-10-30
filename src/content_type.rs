#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ContentType {
    AppJSON = 0x00,
}

impl ContentType {
    /**
     * @brief Converts a byte into a ContentType variant.
     *
     * @param byte The byte representing the content type.
     * @return Ok(ContentType) if conversion succeeds, otherwise
     *         Err(&'static str) describing the parsing error.
     */
    pub fn from_bytes(byte: u8) -> Result<ContentType, &'static str> {
        match byte {
            0 => Ok(ContentType::AppJSON),
            _ => Err("Unknown content type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_content_type() -> Result<(), &'static str> {
        assert_eq!(ContentType::from_bytes(0)?, ContentType::AppJSON);
        Ok(())
    }
}

use crate::{
    MAGIC, RESPONSE_HEADER_SIZE, VERSION, content_type::ContentType, status_code::StatusCode,
};

#[derive(Debug, Clone, Copy)]
pub struct ResponseHeader {
    pub magic: [u8; 4],
    pub version: u8,
    pub content_type: ContentType,
    pub request_id: u32,
    pub status_code: StatusCode,
    pub body_length: u64,
}

#[derive(Debug)]
pub struct Response {
    pub header: ResponseHeader,
    pub body: String,
}

impl Response {
    /**
     * @brief Deserializes a byte slice into a Response.
     *
     * Parses the provided byte data and constructs a Response instance.
     *
     * @param data The input byte slice containing the serialized Response.
     * @return Ok(Response) if parsing succeeds, otherwise Err(&'static str)
     *         describing the parsing error.
     */
    pub fn from_bytes(data: &[u8]) -> Result<Response, &'static str> {
        // Header Check
        if data.len() < RESPONSE_HEADER_SIZE {
            return Err("Incomplete header");
        }

        // Magic Check
        if data[0..4] != MAGIC {
            return Err("Invalid Magic");
        }

        // Version Check
        if data[4] != VERSION {
            return Err("Outdated version");
        }

        let content_type = ContentType::from_bytes(data[5])?;
        let status_code = StatusCode::from_bytes([data[10], data[11]])?;

        let request_id: u32 = u32::from_be_bytes([data[6], data[7], data[8], data[9]]);
        let body_length: u64 = u64::from_be_bytes([
            data[12], data[13], data[14], data[15], data[16], data[17], data[18], data[19],
        ]);

        // Body
        let mut body = Vec::with_capacity(body_length as usize);
        for i in 0..body_length as usize {
            body.push(data[20 + i]);
        }
        let body =
            String::from_utf8(body).map_err(|_| "Failed to transform body Vec to String!")?;

        Ok(Response {
            header: ResponseHeader {
                magic: MAGIC,
                version: VERSION,
                content_type: content_type,
                request_id: request_id,
                status_code: status_code,
                body_length: body_length,
            },
            body: body,
        })
    }

    /**
     * @brief Serializes the Response into a byte vector.
     *
     * @return Ok(Vec<u8>) containing the serialized Response,
     *         otherwise Err(&'static str) describing the serialization error.
     */
    pub fn to_bytes(&self) -> Result<Vec<u8>, &'static str> {
        let body = self.body.as_bytes();

        let mut buffer: Vec<u8> = Vec::with_capacity(RESPONSE_HEADER_SIZE + body.len());

        buffer.extend_from_slice(&MAGIC);
        buffer.push(VERSION);
        buffer.push(self.header.content_type as u8);
        buffer.extend_from_slice(&self.header.request_id.to_be_bytes());
        buffer.extend_from_slice(&(self.header.status_code as u16).to_be_bytes());
        buffer.extend_from_slice(&(body.len() as u64).to_be_bytes());
        buffer.extend_from_slice(body);

        Ok(buffer)
    }

    /**
     * @brief Prints the Response contents to the standard output.
     *
     * Displays the Response fields in a human-readable format.
     *
     * @return Nothing.
     */
    pub fn print(&self) -> () {
        println!("---------- New response ----------");
        println!("Magic: {:?}", self.header.magic);
        println!("Version: {}", self.header.version);
        println!("Content-type: {:?}", self.header.content_type);
        println!("Request ID: {}", self.header.request_id);
        println!("Status code: {:?}", self.header.status_code);
        println!("Body length {}", self.header.body_length);
        println!("Body: {}", self.body);
        println!("---------------------------------");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bytes() -> Result<(), &'static str> {
        let data: [u8; 31] = [
            0x5A, 0x65, 0x6E, 0x21, // Magic: "Zen!"
            0x01, // Version: 1
            0x00, // ContentType::AppJSON = 0x00
            0x00, 0x00, 0x00, 0x2A, // Request ID: 42
            0x00, 0x00, // StatusCode: 0 (0x0000)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0B, // Body length: 11
            0x7B, 0x22, 0x6F, 0x6B, 0x22, 0x3A, 0x74, 0x72, 0x75, 0x65,
            0x7D, // Body: {"ok":true}
        ];

        let obj: Response = Response::from_bytes(&data)?;

        assert_eq!(obj.header.magic, MAGIC);
        assert_eq!(obj.header.version, VERSION);
        assert_eq!(obj.header.content_type, ContentType::AppJSON);
        assert_eq!(obj.header.request_id, 42);
        assert_eq!(obj.header.status_code, StatusCode::Success);
        assert_eq!(obj.header.body_length, 11);
        assert_eq!(obj.body, String::from("{\"ok\":true}"));

        Ok(())
    }

    #[test]
    fn to_bytes() -> Result<(), &'static str> {
        let expected_data: [u8; 31] = [
            0x5A, 0x65, 0x6E, 0x21, // Magic: "Zen!"
            0x01, // Version: 1
            0x00, // ContentType::AppJSON = 0x00
            0x00, 0x00, 0x00, 0x2A, // Request ID: 42
            0x00, 0x00, // StatusCode: 0 (0x0000)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0B, // Body length: 11
            0x7B, 0x22, 0x6F, 0x6B, 0x22, 0x3A, 0x74, 0x72, 0x75, 0x65,
            0x7D, // Body: {"ok":true}
        ];

        let res: Response = Response {
            header: ResponseHeader {
                magic: MAGIC,
                version: VERSION,
                content_type: ContentType::AppJSON,
                request_id: 42,
                status_code: StatusCode::Success,
                body_length: 11,
            },
            body: String::from("{\"ok\":true}"),
        };
        let buffer = res.to_bytes()?;

        assert_eq!(buffer, expected_data);

        Ok(())
    }
}

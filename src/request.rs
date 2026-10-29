use crate::{MAGIC, REQUEST_HEADER_SIZE, VERSION, content_type::ContentType, methods::Methods};

use std::io::Read;

#[derive(Debug, Clone, Copy)]
pub struct RequestHeader {
    pub magic: [u8; 4],
    pub version: u8,
    pub method: Methods,
    pub content_type: ContentType,
    pub request_id: u32,
    pub path_length: u16,
    pub body_length: u64,
}

#[derive(Debug, Clone)]
pub struct Request {
    pub header: RequestHeader,
    pub path: String,
    pub body: String,
}

impl Request {
    pub fn from_bytes<R: Read>(stream: &mut R, bytes: &[u8]) -> Result<Request, &'static str> {
        if bytes.len() < REQUEST_HEADER_SIZE {
            return Err("Request size is smaller than expected!");
        }

        if bytes[0..4] != MAGIC {
            return Err("Unmatched magic!");
        }

        if bytes[4] != VERSION {
            return Err("Incompatible version!");
        }

        let method = match bytes[5] {
            0 => Methods::GET,
            1 => Methods::POST,
            2 => Methods::UPDATE,
            3 => Methods::DELETE,
            _ => return Err("Unknown method!"),
        };

        let content_type = match bytes[6] {
            0 => ContentType::AppJSON,
            _ => return Err("Unknown content type!"),
        };

        let request_id: u32 = u32::from_be_bytes(bytes[7..11].try_into().unwrap());
        let path_length: u16 = u16::from_be_bytes(bytes[11..13].try_into().unwrap());
        let body_length: u64 = u64::from_be_bytes(bytes[13..21].try_into().unwrap());

        let path_len: usize = usize::from(path_length);
        let body_len: usize = usize::try_from(body_length).map_err(|_| "Body too large!")?;
        if body_len > method.max_body_size() {
            return Err("Body too large!");
        }

        let total: usize = path_len.checked_add(body_len).ok_or("Request too large!")?;

        let mut buffer: Vec<u8> = vec![0u8; total];
        stream
            .read_exact(&mut buffer)
            .map_err(|_| "Failed to read path and body!")?;

        let path: String = String::from_utf8(buffer[0..path_len].to_vec())
            .map_err(|_| "Invalid UTF-8 in path!")?;

        let body: String =
            String::from_utf8(buffer[path_len..].to_vec()).map_err(|_| "Invalid UTF-8 in body!")?;

        Ok(Request {
            header: RequestHeader {
                magic: MAGIC,
                version: VERSION,
                method: method,
                content_type: content_type,
                request_id: request_id,
                path_length: path_length,
                body_length: body_length,
            },
            path: path,
            body: body,
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, &'static str> {
        let body = self.body.as_bytes();
        let path = self.path.as_bytes();

        let mut buffer: Vec<u8> = Vec::with_capacity(REQUEST_HEADER_SIZE + body.len() + path.len());

        buffer.extend_from_slice(&MAGIC);
        buffer.push(VERSION);
        buffer.push(self.header.method as u8);
        buffer.push(self.header.content_type as u8);
        buffer.extend_from_slice(&self.header.request_id.to_be_bytes());
        buffer.extend_from_slice(&(path.len() as u16).to_be_bytes());
        buffer.extend_from_slice(&(body.len() as u64).to_be_bytes());
        buffer.extend_from_slice(path);
        buffer.extend_from_slice(body);

        Ok(buffer)
    }

    pub fn print(&self) -> () {
        println!("---------- New request ----------");
        println!("Magic: {:?}", self.header.magic);
        println!("Version: {}", self.header.version);
        println!("Method: {:?}", self.header.method);
        println!("Content-type: {:?}", self.header.content_type);
        println!("Request ID: {}", self.header.request_id);
        println!("Path length {}", self.header.path_length);
        println!("Body length {}", self.header.body_length);
        println!("Path: {}", self.path);
        println!("Body: {}", self.body);
        println!("---------------------------------");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn from_bytes() -> Result<(), &'static str> {
        let data: [u8; 24] = [
            0x5A, 0x65, 0x6E, 0x21, // 0–3: magic
            0x01, // 4: version
            0x01, // 5: method
            0x00, // 6: content type
            0x00, 0x00, 0x00, 0x2A, // 7–10: request_id
            0x00, 0x01, // 11–12: path_length = 1
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, // 13–20: body_length = 2
            0x2F, // 21: '/'
            0x7B, 0x7D, // 22–23: '{}'
        ];

        let (head, rest) = data.split_at(REQUEST_HEADER_SIZE);

        let mut cursor: Cursor<&[u8]> = Cursor::new(rest);

        let obj = Request::from_bytes(&mut cursor, &head)?;

        assert_eq!(obj.header.magic, MAGIC);
        assert_eq!(obj.header.version, 1);
        assert_eq!(obj.header.method, Methods::POST);
        assert_eq!(obj.header.content_type, ContentType::AppJSON);
        assert_eq!(obj.header.request_id, 42);
        assert_eq!(obj.header.path_length, 1);
        assert_eq!(obj.header.body_length, 2);
        assert_eq!(obj.path, String::from("/"));
        assert_eq!(obj.body, String::from("{}"));

        Ok(())
    }

    #[test]
    fn as_bytes() -> Result<(), &'static str> {
        let expected_data: [u8; 24] = [
            0x5A, 0x65, 0x6E, 0x21, // 0–3: magic
            0x01, // 4: version
            0x00, // 5: method
            0x00, // 6: content type
            0x00, 0x00, 0x00, 0x2A, // 7–10: request_id
            0x00, 0x01, // 11–12: path_length = 1
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, // 13–20: body_length = 2
            0x2F, // 21: '/'
            0x7B, 0x7D, // 22–23: '{}'
        ];

        let req = Request {
            header: RequestHeader {
                magic: MAGIC,
                version: VERSION,
                method: Methods::GET,
                content_type: ContentType::AppJSON,
                request_id: 42,
                path_length: 1,
                body_length: 2,
            },
            path: "/".to_string(),
            body: "{}".to_string(),
        };

        let buffer = req.to_bytes()?;

        assert_eq!(buffer, expected_data);

        Ok(())
    }
}

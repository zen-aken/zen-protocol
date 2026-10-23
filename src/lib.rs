const MAGIC: [u8; 4] = *b"Zen!";
const VERSION: u8 = 1;
const REQUEST_HEADER_SIZE: usize = 21;
const RESPONSE_HEADER_SIZE: usize = 20;

#[derive(Debug)]
pub struct RequestHeader {
    pub magic: [u8; 4],
    pub version: u8,
    pub method: Methods,
    pub content_type: ContentType,
    pub request_id: u32,
    pub path_length: u16,
    pub body_length: u64,
}

#[derive(Debug)]
pub struct Request {
    pub header: RequestHeader,
    pub path: Vec<u8>,
    pub body: Vec<u8>,
}

#[derive(Debug)]
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
    pub body: Vec<u8>,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum Methods {
    GET = 0x00,
    POST = 0x01,
    UPDATE = 0x02,
    DELETE = 0x03,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum ContentType {
    AppJSON = 0x00,
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq)]
pub enum StatusCode {
    Success = 0,
    UnknownError = 999,
}

fn parse_method(byte: u8) -> Result<Methods, &'static str> {
    match byte {
        0 => Ok(Methods::GET),
        1 => Ok(Methods::POST),
        2 => Ok(Methods::UPDATE),
        3 => Ok(Methods::DELETE),
        _ => Err("Unknown method"),
    }
}

fn parse_content_type(byte: u8) -> Result<ContentType, &'static str> {
    match byte {
        0 => Ok(ContentType::AppJSON),
        _ => Err("Unknown content type"),
    }
}

fn parse_status_code(byte: [u8; 2]) -> Result<StatusCode, &'static str> {
    match u16::from_be_bytes(byte) {
        0 => Ok(StatusCode::Success),
        _ => Err("Unknown status code"),
    }
}

pub fn parse_request(data: &[u8]) -> Result<Request, &'static str> {
    // Header Check
    if data.len() < REQUEST_HEADER_SIZE {
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

    // Method Check
    let method = match parse_method(data[5]) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    // Content Type Check
    let content_type = match parse_content_type(data[6]) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    // Header variables
    let request_id: u32 = u32::from_be_bytes([data[7], data[8], data[9], data[10]]);
    let path_length: u16 = u16::from_be_bytes([data[11], data[12]]);
    let body_length: u64 = u64::from_be_bytes([
        data[13], data[14], data[15], data[16], data[17], data[18], data[19], data[20],
    ]);

    // body and path
    let mut path: Vec<u8> = Vec::with_capacity(path_length as usize);
    for i in 0..usize::from(path_length) {
        path.push(data[21 + i]);
    }

    let mut body: Vec<u8> = Vec::with_capacity(body_length as usize);
    for i in 0..body_length as usize {
        body.push(data[21 + path_length as usize + i]);
    }

    Ok(Request {
        header: {
            RequestHeader {
                magic: MAGIC,
                version: VERSION,
                method: method,
                content_type: content_type,
                request_id,
                path_length,
                body_length,
            }
        },
        path,
        body,
    })
}

pub fn parse_response(data: &[u8]) -> Result<Response, &'static str> {
    // Header Check
    if data.len() < REQUEST_HEADER_SIZE {
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

    // Content Type Check
    let content_type = match parse_content_type(data[5]) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    // Status Code
    let status_code = match parse_status_code([data[10], data[11]]) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    let request_id: u32 = u32::from_be_bytes([data[6], data[7], data[8], data[9]]);
    let body_length: u64 = u64::from_be_bytes([
        data[12], data[13], data[14], data[15], data[16], data[17], data[18], data[19],
    ]);

    // Body
    let mut body = Vec::with_capacity(body_length as usize);
    for i in 0..body_length as usize {
        body.push(data[20 + i]);
    }

    Ok(Response {
        header: ResponseHeader {
            magic: MAGIC,
            version: VERSION,
            content_type,
            request_id,
            status_code,
            body_length,
        },
        body,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_method() -> Result<(), &'static str> {
        assert_eq!(parse_method(0).unwrap(), Methods::GET);
        assert_eq!(parse_method(1).unwrap(), Methods::POST);
        assert_eq!(parse_method(2).unwrap(), Methods::UPDATE);
        assert_eq!(parse_method(3).unwrap(), Methods::DELETE);
        Ok(())
    }

    #[test]
    fn test_parse_content_type() -> Result<(), &'static str> {
        assert_eq!(parse_content_type(0)?, ContentType::AppJSON);
        Ok(())
    }

    #[test]
    fn test_parse_status_code() -> Result<(), &'static str> {
        assert_eq!(parse_status_code([0, 0])?, StatusCode::Success);
        Ok(())
    }

    #[test]
    fn test_parse_request() -> Result<(), &'static str> {
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
        let obj = parse_request(&data)?;

        assert_eq!(obj.header.magic, MAGIC);
        assert_eq!(obj.header.version, 1);
        assert_eq!(obj.header.method, Methods::POST);
        assert_eq!(obj.header.content_type, ContentType::AppJSON);
        assert_eq!(obj.header.request_id, 42);
        assert_eq!(obj.header.path_length, 1);
        assert_eq!(obj.header.body_length, 2);
        assert_eq!(obj.path, b"/");
        assert_eq!(obj.body, b"{}");

        Ok(())
    }

    #[test]
    fn test_parse_response() -> Result<(), &'static str> {
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

        let obj = parse_response(&data)?;

        assert_eq!(obj.header.magic, MAGIC);
        assert_eq!(obj.header.version, VERSION);
        assert_eq!(obj.header.content_type, ContentType::AppJSON);
        assert_eq!(obj.header.request_id, 42);
        assert_eq!(obj.header.status_code, StatusCode::Success);
        assert_eq!(obj.header.body_length, 11);
        assert_eq!(obj.body, b"{\"ok\":true}");

        Ok(())
    }
}

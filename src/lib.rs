pub mod content_type;
pub mod error;
pub mod methods;
pub mod request;
pub mod response;
pub mod status_code;

pub const MAGIC: [u8; 4] = *b"Zen!";
pub const VERSION: u8 = 1;
pub const REQUEST_HEADER_SIZE: usize = 21;
pub const RESPONSE_HEADER_SIZE: usize = 20;

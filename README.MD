# Zen Protocol

Zen Protocol is a custom binary protocol designed for structured request-response communication. It uses fixed-size headers, explicit field encodings, and length-prefixed payloads.

## Message Format

Zen Protocol supports two message types:

- **Request** — carries a method, content type, request identifier, path, and optional body.
- **Response** — carries a content type, request identifier, status code, and optional body.

All multi-byte integers use **big-endian byte order**.

## Request

A request consists of a 21-byte header, followed by the path and body.

### Request Header

| Field | Type | Size | Description |
|---|---|---:|---|
| `magic` | `[u8; 4]` | 4 bytes | Protocol identifier: `Zen!` |
| `version` | `u8` | 1 byte | Protocol version |
| `method` | `u8` | 1 byte | Request method |
| `content_type` | `u8` | 1 byte | Body content type |
| `request_id` | `u32` | 4 bytes | Identifies the request |
| `path_length` | `u16` | 2 bytes | Path length in bytes |
| `body_length` | `u64` | 8 bytes | Body length in bytes |
| **Total** | | **21 bytes** | |

### Request Structure

```rust
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
```

### Request Methods

Methods are encoded as unsigned 8-bit integers.

| Method | Value |
|---|---:|
| `GET` | `0x00` |
| `POST` | `0x01` |
| `UPDATE` | `0x02` |
| `DELETE` | `0x03` |

### Request Layout

```text
+---------------------------+
| Magic          4 bytes    |
| Version        1 byte     |
| Method         1 byte     |
| Content Type   1 byte     |
| Request ID     4 bytes    |
| Path Length    2 bytes    |
| Body Length    8 bytes    |
+---------------------------+
| Path           N bytes    |
+---------------------------+
| Body           M bytes    |
+---------------------------+
```

The path begins immediately after the header. The body begins immediately after the path.

The total message length is:

`21 + path_length + body_length` bytes.

## Response

A response consists of a 20-byte header, followed by the body.

### Response Header

| Field | Type | Size | Description |
|---|---|---:|---|
| `magic` | `[u8; 4]` | 4 bytes | Protocol identifier: `Zen!` |
| `version` | `u8` | 1 byte | Protocol version |
| `content_type` | `u8` | 1 byte | Body content type |
| `request_id` | `u32` | 4 bytes | Matches the corresponding request |
| `status_code` | `u16` | 2 bytes | Indicates the result |
| `body_length` | `u64` | 8 bytes | Body length in bytes |
| **Total** | | **20 bytes** | |

### Response Structure

```rust
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
```

### Response Layout

```text
+---------------------------+
| Magic          4 bytes    |
| Version        1 byte     |
| Content Type   1 byte     |
| Request ID     4 bytes    |
| Status Code    2 bytes    |
| Body Length    8 bytes    |
+---------------------------+
| Body           N bytes    |
+---------------------------+
```

The total response length is:

`20 + body_length` bytes.

## Content Types

Content types are encoded as unsigned 8-bit integers.

| Content Type | Value | Description |
|---|---:|---|
| `AppJSON` | `0x00` | JSON payload |

## Status Codes

Status codes are encoded as unsigned 16-bit integers.

| Status Code | Value | Description |
|---|---:|---|
| `Success` | `0` | The request completed successfully |
| `UnknownError` | `999` | An unspecified error occurred |

## Implementation Requirements

A compliant implementation should:

1. Validate the magic bytes before processing a message.
2. Validate the protocol version.
3. Reject unknown methods, content types, and status codes.
4. Decode multi-byte integers using big-endian byte order.
5. Validate message lengths before accessing or allocating payload data.
6. Enforce a configurable maximum message size.
7. Preserve the request identifier in the corresponding response.
8. Return parsing errors instead of panicking on malformed input.

The binary format must be serialized and parsed explicitly. Rust's in-memory struct layout is not the wire format; `#[repr]` on an enum does not automatically serialize a complete message.

## Current Scope

The protocol currently defines:

- Binary request and response messages
- Request methods
- Content types
- Status codes
- Length-prefixed paths and bodies

Features such as authentication, encryption, connection framing, streaming, and additional content types are not yet specified.

## License

MIT License. See the [LICENSE](LICENSE) file for details.

Copyright (c) 2026 Your Name
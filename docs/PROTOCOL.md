# Zen Protocol Specification

This document defines the binary message format and encoding rules used by Zen Protocol.

## 1. Overview

Zen Protocol is a binary request-response protocol.

It defines two message types:

- **Request**: Carries a method, content type, request identifier, path, and body.
- **Response**: Carries a content type, request identifier, status code, and body.

### Encoding Rules

- All multi-byte integers use big-endian byte order.
- Strings are encoded using UTF-8.
- Length fields represent bytes, not characters.
- All messages begin with the protocol magic identifier `Zen!`.
- Rust struct memory layouts are not used as wire representations. Fields are encoded explicitly.

## 2. Request

A request consists of a fixed-size header, followed by a path and an optional body.

### 2.1 Request Header

Header size: **21 bytes**

| Field | Type | Size | Description |
|---|---|---:|---|
| `magic` | `[u8; 4]` | 4 | Protocol identifier (`Zen!`) |
| `version` | `u8` | 1 | Protocol version |
| `method` | `u8` | 1 | Request method |
| `content_type` | `u8` | 1 | Body content type |
| `request_id` | `u32` | 4 | Request identifier |
| `path_length` | `u16` | 2 | Path length in bytes |
| `body_length` | `u64` | 8 | Body length in bytes |
| **Total** | | **21** | |

### 2.2 Request Layout

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

The path immediately follows the header. The body immediately follows the path.

Total request size:

`21 + path_length + body_length` bytes.

### 2.3 Request Methods

Methods are encoded as unsigned 8-bit integers.

| Method | Value |
|---|---:|
| `GET` | `0x00` |
| `POST` | `0x01` |
| `UPDATE` | `0x02` |
| `DELETE` | `0x03` |

## 3. Response

A response consists of a fixed-size header followed by an optional body.

### 3.1 Response Header

Header size: **20 bytes**

| Field | Type | Size | Description |
|---|---|---:|---|
| `magic` | `[u8; 4]` | 4 | Protocol identifier (`Zen!`) |
| `version` | `u8` | 1 | Protocol version |
| `content_type` | `u8` | 1 | Body content type |
| `request_id` | `u32` | 4 | Corresponding request identifier |
| `status_code` | `u16` | 2 | Response status |
| `body_length` | `u64` | 8 | Body length in bytes |
| **Total** | | **20** | |

### 3.2 Response Layout

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

Total response size:

`20 + body_length` bytes.

## 4. Content Types

Content types are encoded as unsigned 8-bit integers.

| Content Type | Value | Description |
|---|---:|---|
| `AppJSON` | `0x00` | JSON payload |

## 5. Status Codes

Status codes are encoded as unsigned 16-bit integers.

| Status Code | Value | Description |
|---|---:|---|
| `Success` | `0` | Request completed successfully |
| `UnknownError` | `999` | An unspecified error occurred |

## 6. Serialization

Messages must be serialized and deserialized explicitly.

### Integer Encoding

Multi-byte integers use big-endian byte order.

For example, the `request_id` value `42` is encoded as:

```text
00 00 00 2A
```

### String Encoding

Paths and bodies use UTF-8 encoding.

The length of a string is its encoded byte length:

```rust
let path_length = path.len();
let body_length = body.len();
```

### Validation

Implementations should:

1. Validate the magic identifier.
2. Validate the protocol version.
3. Reject unknown methods, content types, and status codes.
4. Validate header and payload lengths.
5. Enforce maximum message and body sizes.
6. Validate UTF-8 when decoding strings.
7. Preserve request identifiers in corresponding responses.
8. Return errors instead of panicking on malformed messages.

## 7. Message Handling

A request-response exchange follows this sequence:

```text
Client                  Server
  |                        |
  |------ Request -------->|
  |                        |
  |                 Parse Request
  |                 Process Request
  |                 Create Response
  |                        |
  |<----- Response --------|
  |                        |
```

The server should preserve the incoming `request_id` in its corresponding response.

## 8. Current Scope

The current specification defines:

- Binary request and response messages.
- Fixed-size headers.
- Request methods.
- Content types.
- Status codes.
- Length-prefixed paths and bodies.
- UTF-8 string encoding.
- Big-endian integer encoding.

The following features are not yet specified:

- Authentication and authorization.
- Encryption and secure transport.
- Connection management.
- Streaming and chunked messages.
- Additional content types.
- Protocol version negotiation.

## 9. API Reference

For Rust structures, enums, methods, and function documentation, see the [API Reference](API.md).

## 10. License

MIT License. See the [LICENSE](../LICENSE) file for details.
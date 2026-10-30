## API Reference

### Request

| Function | Description |
|---|---|
| `Request::from_bytes<R: Read>()` | Reads request payload data from a stream and deserializes a Request. |
| `Request::to_bytes()` | Serializes a Request into a byte vector. |
| `Request::print()` | Prints request information in a human-readable format. |

### Response

| Function | Description |
|---|---|
| `Response::from_bytes()` | Deserializes a byte slice into a Response. |
| `Response::to_bytes()` | Serializes a Response into a byte vector. |
| `Response::print()` | Prints response information in a human-readable format. |

### Methods

| Function | Description |
|---|---|
| `Methods::parse()` | Converts a byte into a Methods variant. |
| `Methods::max_body_size()` | Returns the maximum allowed body size for the specified method. |

### ContentType

| Function | Description |
|---|---|
| `ContentType::from_bytes()` | Converts a byte into a ContentType variant. |

### StatusCode

| Function | Description |
|---|---|
| `StatusCode::from_bytes()` | Converts a two-byte array into a StatusCode variant. |

### Return Values

| Type | Description |
|---|---|
| `Result<T, &'static str>` | Returns the parsed or serialized result, or a static error message. |
| `Vec<u8>` | Serialized binary message. |
| `usize` | Maximum body size in bytes. |
| `()` | No return value. |

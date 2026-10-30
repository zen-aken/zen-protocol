# Zen Protocol

Zen Protocol is a custom binary protocol written in Rust, designed for structured request-response communication.

It provides a lightweight and extensible communication format using fixed-size headers, explicit field encoding, and length-prefixed payloads.

## Features

- Custom binary message format.
- Request-response communication.
- Explicit big-endian integer encoding.
- Length-prefixed paths and payloads.
- UTF-8 string support.
- Configurable body size limits.
- Explicit serialization and deserialization.
- Extensible method, content type, and status code definitions.

## Documentation

| Document | Description |
|---|---|
| [Protocol Specification](docs/PROTOCOL.md) | Binary message format, headers, field definitions, and encoding rules. |
| [API Reference](docs/API.md) | Public Rust structures, enums, methods, and functions. |

## Project Structure

```text
zen-protocol/
├── docs/
│   ├── API.md
│   └── PROTOCOL.md
├── src/
│   └── lib.rs
├── Cargo.toml
├── LICENSE
└── README.md
```

## Usage

Add Zen Protocol to your project's `Cargo.toml`:

```toml
[dependencies]
zen-protocol = { git = "https://github.com/zen-aken/zen-protocol" }
```

Example:

```rust
use zen_protocol::{Request, RequestHeader, Methods, ContentType};

let request = Request {
    header: RequestHeader {
        magic: *b"Zen!",
        version: 1,
        method: Methods::GET,
        content_type: ContentType::AppJSON,
        request_id: 42,
        path_length: 5,
        body_length: 2,
    },
    path: "/test".to_string(),
    body: "{}".to_string(),
};

let bytes = request.to_bytes().unwrap();
```

See the [API Reference](docs/API.md) for available functions and the [Protocol Specification](docs/PROTOCOL.md) for binary encoding details.

## License

MIT License. See [LICENSE](LICENSE) for details.

Copyright (c) 2026 Zen
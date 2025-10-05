# Data Source IRWIN

A Rust library for integrating with the IRWIN (Integrated Reporting of Wildfire Information) Incidents API.

## Overview

This library provides a stateless client for querying wildland fire incident data from the IRWIN API. It's designed to be simple and focused, with no built-in state management - perfect for integration into larger systems that handle their own state.

## Features

- **Stateless Design**: No internal state management - each request is independent
- **Token-based Authentication**: Automatic token generation and management
- **Query Builder Pattern**: Fluent API for building complex incident queries
- **IRWIN Extensions Support**: Full support for IRWIN-specific query parameters
- **Multiple Environments**: Support for TEST, OAT, and PROD environments
- **Async/Await**: Built on tokio for high-performance async operations

## IRWIN API Support

The client supports the core IRWIN Incidents API features:

- Query incidents with custom WHERE clauses
- Include/exclude resources, relationships, and metadata
- Incremental synchronization using ModifiedOnDateTime
- Token-based authentication with referer validation

## Usage

### Basic Setup

```rust
use data_source_irwin::{IrwinClient, IrwinConfig, types::environments};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = IrwinConfig::new(
        environments::PROD.to_string(),
        "your_username".to_string(),
        "your_password".to_string(),
        "your_referer".to_string(),
    );

    let client = IrwinClient::new(config)?;
    Ok(())
}
```

### Query Incidents

```rust
use data_source_irwin::{IncidentQueryBuilder, types::environments};

let query = IncidentQueryBuilder::new()
    .where_clause("IsValid=1 AND IncidentTypeKind='FI'")
    .out_fields("IrwinID,IncidentName,UniqueFireIdentifier")
    .return_geometry("true")
    .include_resources(true)
    .include_relationships(true)
    .build();

let response = client.query_incidents(&query).await?;
```

### Query by IDs

```rust
let irwin_ids = vec!["12345".to_string(), "67890".to_string()];
let response = client.query_by_irwin_ids(&irwin_ids).await?;
```

### Incremental Sync

```rust
let last_sync = 1640995200000; // Unix timestamp in milliseconds
let response = client.sync_since(last_sync).await?;
```

## Architecture

The library is organized into three main modules:

- **`lib.rs`**: Main client implementation and public API
- **`error.rs`**: Custom error types and conversions
- **`types.rs`**: Data structures, configuration, and constants

## Dependencies

- `reqwest`: HTTP client for API requests
- `serde`: Serialization/deserialization
- `tokio`: Async runtime (optional, feature-gated)
- `url`: URL parsing utilities

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Code Quality

```bash
cargo fmt
cargo clippy
```

## Security Notes

- Credentials are stored in memory only during client lifetime
- No persistent storage of tokens or credentials
- Referer validation is enforced as per IRWIN requirements
- Tokens expire after 60 minutes as per API specification

## Contributing

1. Follow Rust coding standards
2. Add tests for new functionality
3. Update documentation for API changes
4. Ensure all tests pass before submitting

## License

This project is licensed under the MIT License.

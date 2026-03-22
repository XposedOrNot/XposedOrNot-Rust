<p align="center">
  <a href="https://xposedornot.com">
    <img src="https://xposedornot.com/static/logos/xon.png" alt="XposedOrNot" width="200">
  </a>
</p>

<h1 align="center">xposedornot</h1>

<p align="center">
  Official Rust SDK for the <a href="https://xposedornot.com">XposedOrNot</a> API<br>
  <em>Check if your email or password has been exposed in data breaches</em>
</p>

<p align="center">
  <a href="https://crates.io/crates/xposedornot"><img src="https://img.shields.io/crates/v/xposedornot.svg" alt="crates.io"></a>
  <a href="https://docs.rs/xposedornot"><img src="https://img.shields.io/docsrs/xposedornot" alt="docs.rs"></a>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT"></a>
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/MSRV-1.70-blue.svg" alt="MSRV: 1.70"></a>
</p>

---

> **Note:** This SDK uses the free public API from [XposedOrNot.com](https://xposedornot.com) -- a free service to check if your email has been compromised in data breaches. Visit the [XposedOrNot website](https://xposedornot.com) to learn more about the service and check your email manually.

---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Requirements](#requirements)
- [Quick Start](#quick-start)
- [API Reference](#api-reference)
  - [check_email](#check_emailemail)
  - [get_breaches](#get_breachesdomain)
  - [breach_analytics](#breach_analyticsemail)
  - [check_password](#check_passwordpassword)
- [Error Handling](#error-handling)
- [Rate Limits](#rate-limits)
- [Configuration](#configuration)
- [Contributing](#contributing)
- [License](#license)
- [Links](#links)

---

## Features

- **Email Breach Check** -- Query the free or Plus API for email exposures
- **Breach Listing** -- List all known breaches with optional domain filtering
- **Breach Analytics** -- Detailed summaries, metrics, and paste data for an email
- **Password Check** -- k-anonymity via Keccak-512; the full password never leaves the client
- **Async/Await** -- Built on `reqwest` and `tokio` for non-blocking I/O
- **Rate Limiting** -- Automatic client-side throttling (1 req/sec for the free API)
- **Retry Logic** -- Exponential backoff on HTTP 429 responses
- **Configurable** -- Builder pattern for timeout, retries, custom headers, and API key

## Installation

```bash
cargo add xposedornot
```

Or add it directly to your `Cargo.toml`:

```toml
[dependencies]
xposedornot = "1.0"
```

## Requirements

- **Rust 1.70** or higher
- A **tokio** async runtime (the crate depends on `tokio` for rate limiting and timing)

## Quick Start

```rust
use xposedornot::Client;

#[tokio::main]
async fn main() -> Result<(), xposedornot::Error> {
    let client = Client::builder().build()?;

    // Check if an email has been breached
    let result = client.check_email("test@example.com").await?;
    println!("{:?}", result);

    // Check password exposure (password is hashed locally)
    let pw = client.check_password("password123").await?;
    println!("Seen {} times", pw.search_pass_anon.count);

    Ok(())
}
```

## API Reference

### Constructor

```rust
use xposedornot::Client;

let client = Client::builder()
    .timeout_secs(60)
    .max_retries(5)
    .build()?;
```

See [Configuration](#configuration) for all builder options.

### Methods

#### `check_email(email)`

Check if an email address has been exposed in any data breaches.

When the client has an API key, the Plus API is used and returns detailed breach records. Without an API key the free API returns a simple list of breach names.

```rust
// Free API
let client = Client::builder().build()?;
let result = client.check_email("user@example.com").await?;
println!("{:?}", result);

// Plus API -- returns detailed breach records
let client = Client::builder()
    .api_key("your-api-key")
    .build()?;
let result = client.check_email("user@example.com").await?;
println!("{:?}", result);
```

**Returns:** `EmailCheckResult` -- an enum with `Free(FreeEmailCheckResponse)` or `Plus(PlusEmailCheckResponse)` variants depending on whether an API key is configured.

#### `get_breaches(domain)`

List all known data breaches, optionally filtered by domain.

```rust
// Get all breaches
let all = client.get_breaches(None).await?;

// Filter by domain
let filtered = client.get_breaches(Some("adobe.com")).await?;

for breach in &filtered.exposed_breaches {
    println!("{} - {} records", breach.breach_id, breach.exposed_records);
}
```

**Returns:** `BreachListResponse` containing a `Vec<BreachRecord>` with fields:
- `breach_id` -- Unique identifier
- `breached_date` -- Date of the breach
- `domain` -- Associated domain
- `industry` -- Industry category
- `exposed_data` -- Types of data exposed
- `exposed_records` -- Number of records exposed
- `verified` -- Whether the breach is verified

#### `breach_analytics(email)`

Get detailed breach analytics for an email address, including breach summaries, metrics, and paste exposures.

```rust
let analytics = client.breach_analytics("user@example.com").await?;

println!("Breaches: {:?}", analytics.exposed_breaches);
println!("Summary: {:?}", analytics.breaches_summary);
println!("Metrics: {:?}", analytics.breach_metrics);
println!("Pastes: {:?}", analytics.exposed_pastes);
```

**Returns:** `BreachAnalyticsResponse` with fields:
- `exposed_breaches` -- Breach details
- `breaches_summary` -- Aggregated summary
- `breach_metrics` -- Analytical metrics
- `pastes_summary` -- Paste summary
- `exposed_pastes` -- List of paste exposures

#### `check_password(password)`

Check if a password has been seen in known breaches. The password is hashed locally using **Keccak-512** and only the first 10 hex characters of the digest are sent to the API (k-anonymity). The full password **never** leaves the client.

```rust
let result = client.check_password("mysecretpassword").await?;
println!("Exposure count: {}", result.search_pass_anon.count);
```

**Returns:** `PasswordCheckResponse` with a `search_pass_anon` field containing:
- `anon` -- The anonymous hash portion
- `char` -- Character composition breakdown (e.g., `"D:3;A:8;S:0;L:11"`)
- `count` -- Number of times the password has been seen

## Error Handling

All methods return `Result<T, xposedornot::Error>`. Use pattern matching on the `Error` enum to handle specific failure modes:

```rust
use xposedornot::{Client, Error};

let client = Client::builder().build()?;

match client.check_email("user@example.com").await {
    Ok(result) => println!("{:?}", result),
    Err(Error::Validation { message }) => {
        eprintln!("Invalid input: {message}");
    }
    Err(Error::RateLimit { message }) => {
        eprintln!("Rate limited: {message}");
    }
    Err(Error::NotFound { message }) => {
        eprintln!("Not found: {message}");
    }
    Err(Error::Authentication { message }) => {
        eprintln!("Auth failed: {message}");
    }
    Err(Error::Network { source }) => {
        eprintln!("Network error: {source}");
    }
    Err(Error::Api { status_code, message }) => {
        eprintln!("API error ({status_code}): {message}");
    }
}
```

### Error Variants

| Variant | Description |
|---------|-------------|
| `Validation` | Invalid input (e.g., malformed email, empty password) |
| `RateLimit` | API rate limit exceeded (HTTP 429) |
| `NotFound` | Resource not found (HTTP 404) |
| `Authentication` | Invalid or missing API key (HTTP 401/403) |
| `Network` | Connection or transport error (wraps `reqwest::Error`) |
| `Api` | Unexpected API response or server error |

## Rate Limits

The XposedOrNot free API has the following rate limits:

- 2 requests per second
- 50-100 requests per hour
- 100-1000 requests per day

The client enforces **client-side rate limiting** at 1 request per second for the free API. When an API key is configured (Plus API), client-side throttling is disabled.

Server-side 429 responses are retried automatically with exponential backoff (1s, 2s, 4s, ...) up to `max_retries` attempts.

## Configuration

Use the builder pattern to customize the client:

```rust
use xposedornot::Client;

let client = Client::builder()
    .api_key("your-api-key")          // Enable Plus API
    .timeout_secs(60)                 // Request timeout (default: 30s)
    .max_retries(5)                   // Retry attempts on 429 (default: 3)
    .base_url("https://custom.api")   // Override free API base URL
    .header("X-Custom", "value")      // Add custom headers
    .build()?;
```

### Builder Options

| Method | Default | Description |
|--------|---------|-------------|
| `api_key(key)` | `None` | API key for Plus API; disables client-side rate limiting |
| `timeout_secs(secs)` | `30` | Request timeout in seconds |
| `max_retries(n)` | `3` | Max retry attempts on HTTP 429 |
| `base_url(url)` | `https://api.xposedornot.com` | Free API base URL |
| `plus_base_url(url)` | `https://plus-api.xposedornot.com` | Plus API base URL |
| `password_base_url(url)` | `https://passwords.xposedornot.com/api` | Password API base URL |
| `header(name, value)` | -- | Add a custom header to every request |

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Setup

```bash
# Clone the repository
git clone https://github.com/XposedOrNot/XposedOrNot-Rust.git
cd XposedOrNot-Rust

# Build
cargo build

# Run tests
cargo test

# Run clippy
cargo clippy -- -D warnings

# Format
cargo fmt --check
```

## License

MIT -- see the [LICENSE](LICENSE) file for details.

## Links

- [XposedOrNot Website](https://xposedornot.com)
- [API Documentation](https://xposedornot.com/api_doc)
- [crates.io Package](https://crates.io/crates/xposedornot)
- [docs.rs Documentation](https://docs.rs/xposedornot)
- [GitHub Repository](https://github.com/XposedOrNot/XposedOrNot-Rust)
- [XposedOrNot API Repository](https://github.com/XposedOrNot/XposedOrNot-API)

---

<p align="center">
  Made with care by <a href="https://xposedornot.com">XposedOrNot</a>
</p>

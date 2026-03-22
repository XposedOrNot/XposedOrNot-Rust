//! # xposedornot
//!
//! Async Rust client for the [XposedOrNot](https://xposedornot.com) data breach API.
//!
//! ## Overview
//!
//! This crate provides a type-safe, async client for querying the XposedOrNot
//! API to check whether email addresses or passwords have been exposed in
//! known data breaches.
//!
//! ## Features
//!
//! - **Email breach check** (free and Plus API)
//! - **Breach analytics** with detailed summaries and metrics
//! - **Breach listing** with optional domain filtering
//! - **Password exposure check** using anonymized Keccak-512 hashing (k-anonymity)
//! - Client-side rate limiting (1 request/second for free API)
//! - Automatic retry with exponential backoff on HTTP 429
//! - Configurable via builder pattern
//!
//! ## Quick Start
//!
//! ```no_run
//! use xposedornot::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), xposedornot::Error> {
//!     let client = Client::builder().build()?;
//!
//!     // Check if an email has been breached
//!     let result = client.check_email("user@example.com").await?;
//!     println!("{:?}", result);
//!
//!     // Check password exposure (password is hashed locally)
//!     let pw_result = client.check_password("password123").await?;
//!     println!("Seen {} times", pw_result.search_pass_anon.count);
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod endpoints;
pub mod errors;
pub mod models;
pub mod utils;

// Re-export primary types at crate root for convenience.
pub use client::{Client, ClientBuilder, ClientConfig};
pub use errors::Error;
pub use models::{
    BreachAnalyticsResponse, BreachListResponse, BreachRecord, EmailCheckResult,
    FreeEmailCheckResponse, PasswordCheckResponse, PlusBreachDetail, PlusEmailCheckResponse,
};

//! Plus API usage example with API key.
//!
//! Run with: XON_API_KEY=your-key cargo run --example plus_api

use xposedornot::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("XON_API_KEY")
        .expect("Set XON_API_KEY environment variable");

    let client = Client::builder()
        .api_key(api_key)
        .build()?;

    match client.check_email("test@example.com").await? {
        xposedornot::EmailCheckResult::Plus(response) => {
            println!("Status: {}", response.status);
            for breach in &response.breaches {
                println!("  Breach: {} (Domain: {}, Records: {})",
                    breach.breach_id, breach.domain, breach.xposed_records);
            }
        }
        _ => {}
    }

    Ok(())
}

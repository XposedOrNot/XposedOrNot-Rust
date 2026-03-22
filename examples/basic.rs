//! Basic usage example for the XposedOrNot API client.
//!
//! Run with: cargo run --example basic

use xposedornot::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    // Check if an email has been exposed
    match client.check_email("test@example.com").await? {
        xposedornot::EmailCheckResult::Free(response) => {
            let breaches: Vec<String> = response.breaches.into_iter().flatten().collect();
            println!("Found in {} breaches", breaches.len());
        }
        _ => {}
    }

    // Get all known breaches
    let breaches = client.get_breaches(None).await?;
    println!("Total known breaches: {}", breaches.exposed_breaches.len());

    // Check a password (hashed locally, never sent in clear text)
    let pass_result = client.check_password("password123").await?;
    println!("Password exposure count: {}", pass_result.search_pass_anon.count);

    Ok(())
}

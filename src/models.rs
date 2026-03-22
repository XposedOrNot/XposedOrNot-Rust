//! Request and response types for the XposedOrNot API.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Email check responses
// ---------------------------------------------------------------------------

/// Response from the free email check endpoint (`/v1/check-email/{email}`).
///
/// The `breaches` field contains a nested array of breach names.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeEmailCheckResponse {
    /// Nested list of breach names associated with the email.
    pub breaches: Vec<Vec<String>>,
}

/// A single breach record returned by the Plus API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlusBreachDetail {
    /// Unique breach identifier.
    pub breach_id: String,
    /// Date the breach occurred.
    pub breached_date: String,
    /// Logo URL for the breached service.
    pub logo: String,
    /// Password risk level.
    pub password_risk: String,
    /// Whether the breach is searchable.
    pub searchable: String,
    /// Types of data exposed.
    pub xposed_data: String,
    /// Number of records exposed.
    pub xposed_records: u64,
    /// Description of the exposure.
    pub xposure_desc: String,
    /// Domain of the breached service.
    pub domain: String,
}

/// Response from the Plus API email check endpoint (`/v3/check-email/{email}`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlusEmailCheckResponse {
    /// Status of the request (e.g., "success").
    pub status: String,
    /// The email that was checked.
    pub email: String,
    /// Detailed breach records.
    pub breaches: Vec<PlusBreachDetail>,
}

/// Unified email check result that covers both free and Plus API responses.
#[derive(Debug, Clone)]
pub enum EmailCheckResult {
    /// Result from the free API.
    Free(FreeEmailCheckResponse),
    /// Result from the Plus API.
    Plus(PlusEmailCheckResponse),
}

// ---------------------------------------------------------------------------
// Breach listing
// ---------------------------------------------------------------------------

/// A single breach entry from the breach listing endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BreachRecord {
    /// Unique breach identifier.
    #[serde(alias = "breachID")]
    pub breach_id: String,
    /// Date the breach occurred.
    pub breached_date: String,
    /// Domain of the breached service.
    pub domain: String,
    /// Industry category.
    pub industry: String,
    /// Types of data exposed.
    pub exposed_data: String,
    /// Number of records exposed.
    pub exposed_records: u64,
    /// Whether the breach has been verified.
    pub verified: bool,
}

/// Response from the breach listing endpoint (`/v1/breaches`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreachListResponse {
    /// List of exposed breaches.
    #[serde(rename = "exposedBreaches")]
    pub exposed_breaches: Vec<BreachRecord>,
}

// ---------------------------------------------------------------------------
// Breach analytics
// ---------------------------------------------------------------------------

/// Detailed information about a single breach in the analytics response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreachAnalyticsDetail {
    /// The breach name or identifier.
    #[serde(default)]
    pub breach: Option<String>,
    /// Additional fields are captured dynamically.
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Top-level container for exposed breaches in the analytics response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExposedBreaches {
    /// List of breach detail objects.
    pub breaches_details: Vec<serde_json::Value>,
}

/// Summary of breaches in the analytics response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreachesSummary {
    /// Dynamic summary fields.
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Breach metrics in the analytics response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreachMetrics {
    /// Dynamic metrics fields.
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Pastes summary in the analytics response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PastesSummary {
    /// Dynamic summary fields.
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Response from the breach analytics endpoint (`/v1/breach-analytics`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreachAnalyticsResponse {
    /// Exposed breaches with details.
    #[serde(rename = "ExposedBreaches")]
    pub exposed_breaches: ExposedBreaches,
    /// Summary of breaches.
    #[serde(rename = "BreachesSummary")]
    pub breaches_summary: BreachesSummary,
    /// Breach metrics.
    #[serde(rename = "BreachMetrics")]
    pub breach_metrics: BreachMetrics,
    /// Pastes summary.
    #[serde(rename = "PastesSummary")]
    pub pastes_summary: PastesSummary,
    /// List of exposed pastes.
    #[serde(rename = "ExposedPastes")]
    pub exposed_pastes: Vec<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Password check
// ---------------------------------------------------------------------------

/// Inner result of the anonymous password search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordAnonResult {
    /// The anonymous hash portion returned by the API.
    pub anon: String,
    /// Character composition breakdown (e.g., `"D:3;A:8;S:0;L:11"`).
    pub char: String,
    /// Number of times this password has been seen in breaches.
    pub count: String,
}

/// Wrapper for the password search response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordSearchAnon {
    /// The anonymous search result.
    pub anon: String,
    /// Character composition breakdown.
    pub char: String,
    /// Exposure count.
    pub count: String,
}

/// Response from the password check endpoint (`/v1/pass/anon/{hash_prefix}`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordCheckResponse {
    /// The anonymous password search result.
    #[serde(rename = "SearchPassAnon")]
    pub search_pass_anon: PasswordSearchAnon,
}

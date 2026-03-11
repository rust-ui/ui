use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, IntoStaticStr, EnumString, Display)]
pub enum BugType {
    #[default]
    Bug,
    Server,
    Wasm,
    LeptosWarning,
    BrowserWarning,
    Database,
    NotFound,
}

/// Bug report request payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BugReportRequest {
    pub bug_type: BugType,
    pub message: String,
    pub exception_message: Option<String>,
    pub stack_trace: Option<String>,
    pub user_login: Option<String>,
    pub url: Option<String>,
    pub user_agent: Option<String>,
    pub application: String,
}

impl BugReportRequest {
    pub fn new(bug_type: BugType, message: impl Into<String>) -> Self {
        let message = message.into();
        Self {
            bug_type,
            exception_message: Some(message.chars().take(512).collect()),
            message,
            stack_trace: None,
            user_login: None,
            url: None,
            user_agent: None,
            application: "rust-ui".to_string(),
        }
    }

    pub fn with_stack_trace(mut self, stack_trace: impl Into<String>) -> Self {
        self.stack_trace = Some(stack_trace.into());
        self
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }
}

/// Send a bug report to both SQLite (local) and RUSTIFY (remote).
#[cfg(feature = "ssr")]
pub async fn send_bug_report(report: BugReportRequest) -> Result<(), String> {
    // Save to local SQLite first (fast, synchronous)
    if let Err(err) = super::bug_reports_sqlite::save_bug_report(&report) {
        tracing::warn!("Failed to save bug report to SQLite: {err}");
    }

    // Then send to RUSTIFY (may fail if env vars not set or network issues)
    send_bug_report_to_rustify(&report).await
}

/// Send a bug report to RUSTIFY only.
#[cfg(feature = "ssr")]
async fn send_bug_report_to_rustify(report: &BugReportRequest) -> Result<(), String> {
    let Ok(base_url) = std::env::var("RUSTIFY_API_URL") else {
        tracing::debug!("RUSTIFY_API_URL not set, skipping remote bug report");
        return Ok(());
    };

    let Ok(api_key) = std::env::var("BUG_REPORTS_API_KEY") else {
        tracing::debug!("BUG_REPORTS_API_KEY not set, skipping remote bug report");
        return Ok(());
    };

    let url = format!("{}/rust-ui-api/bug-reports", base_url);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("X-API-Key", api_key)
        .header("Content-Type", "application/json")
        .json(&report)
        .send()
        .await
        .map_err(|err| format!("Failed to send bug report: {err}"))?;

    if response.status().is_success() {
        tracing::info!("Bug report sent to RUSTIFY successfully");
        Ok(())
    } else {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        Err(format!("Bug report failed: {status} - {body}"))
    }
}

/// Report a server error (fire-and-forget, spawns a task)
#[cfg(feature = "ssr")]
pub fn report_server_error(message: impl Into<String>, url: Option<String>) {
    let report = BugReportRequest::new(BugType::Server, message).with_url(url.unwrap_or_default());

    tokio::spawn(async move {
        if let Err(err) = send_bug_report(report).await {
            tracing::warn!("Failed to report server error: {err}");
        }
    });
}

/// Report a 404 Not Found (fire-and-forget, spawns a task)
#[cfg(feature = "ssr")]
pub fn report_not_found(url: Option<String>) {
    let report = BugReportRequest::new(BugType::NotFound, "404 Not Found").with_url(url.unwrap_or_default());

    tokio::spawn(async move {
        if let Err(err) = send_bug_report(report).await {
            tracing::warn!("Failed to report 404: {err}");
        }
    });
}

/// Report a database error (fire-and-forget, spawns a task)
#[cfg(feature = "ssr")]
pub fn report_database_error(message: impl Into<String>, url: Option<String>) {
    let report = BugReportRequest::new(BugType::Database, message).with_url(url.unwrap_or_default());

    tokio::spawn(async move {
        if let Err(err) = send_bug_report(report).await {
            tracing::warn!("Failed to report database error: {err}");
        }
    });
}

/* ========================================================== */
/*                     SERVER FUNCTIONS                       */
/* ========================================================== */

use leptos::prelude::*;
use leptos::server_fn::codec::{Json, PostUrl};

/// Server function for client-side bug reporting.
/// Saves to local SQLite and forwards to RUSTIFY.
/// Extracts user agent and URL from HTTP headers for reliability.
#[server(ReportClientBug, "/api", input = PostUrl, output = Json)]
pub async fn report_client_bug(report: BugReportRequest) -> Result<(), ServerFnError> {
    let mut report = report;

    // Extract user agent and URL from HTTP headers (more reliable than client-side)
    if let Some(req) = use_context::<http::request::Parts>() {
        // User agent from User-Agent header
        if let Some(ua) = req.headers.get(http::header::USER_AGENT) {
            report.user_agent = ua.to_str().ok().map(String::from);
        }
        // URL from Referer header (the page that made the request)
        if let Some(referer) = req.headers.get(http::header::REFERER) {
            report.url = referer.to_str().ok().map(String::from);
        }
    }

    send_bug_report(report).await.map_err(|err| ServerFnError::new(format!("Failed to save bug report: {err}")))
}

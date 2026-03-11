//! SQLite storage for bug reports.
//!
//! This provides local, ephemeral storage for bug reports that persists
//! during a deployment session but is lost on redeploy.

#[cfg(feature = "ssr")]
use std::collections::hash_map::DefaultHasher;
#[cfg(feature = "ssr")]
use std::hash::{Hash, Hasher};
#[cfg(feature = "ssr")]
use std::sync::{Mutex, OnceLock};

#[cfg(feature = "ssr")]
use rusqlite::Connection;

#[cfg(feature = "ssr")]
use super::bug_reports::BugReportRequest;

#[cfg(feature = "ssr")]
static DB_CONN: OnceLock<Mutex<Connection>> = OnceLock::new();

/// Local dev: `bug_reports.db` in current directory
#[cfg(feature = "ssr")]
const DB_PATH_DEV: &str = "bug_reports.db";

/// Production: `/tmp/bug_reports.db` (distroless image has writable /tmp)
#[cfg(feature = "ssr")]
const DB_PATH_PROD: &str = "/tmp/bug_reports.db";

#[cfg(feature = "ssr")]
fn get_db_path() -> &'static str {
    if cfg!(debug_assertions) { DB_PATH_DEV } else { DB_PATH_PROD }
}

/// Initialize the SQLite database and create the bug_reports table if it doesn't exist.
#[cfg(feature = "ssr")]
fn init_db() -> Result<(), String> {
    let db_path = get_db_path();
    let conn = Connection::open(db_path).map_err(|err| format!("Failed to open SQLite: {err}"))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS bug_reports (
            id                INTEGER PRIMARY KEY AUTOINCREMENT,
            bug_type          TEXT    NOT NULL,
            similarity_hash   INTEGER NOT NULL,
            message           TEXT    NOT NULL,
            exception_message TEXT,
            stack_trace       TEXT,
            user_login        TEXT,
            url               TEXT,
            user_agent        TEXT,
            application       TEXT    NOT NULL,
            created_at        TEXT    NOT NULL DEFAULT (datetime('now'))
        )",
        [],
    )
    .map_err(|err| format!("Failed to create bug_reports table: {err}"))?;

    // Migration: add similarity_hash column if missing (for existing DBs created before this column)
    // SQLite doesn't have "ADD COLUMN IF NOT EXISTS", so we ignore the error if column exists
    let _ = conn.execute("ALTER TABLE bug_reports ADD COLUMN similarity_hash INTEGER NOT NULL DEFAULT 0", []);

    // Add index on similarity_hash for efficient grouping
    conn.execute("CREATE INDEX IF NOT EXISTS idx_bug_reports_similarity_hash ON bug_reports (similarity_hash)", [])
        .map_err(|err| format!("Failed to create similarity_hash index: {err}"))?;

    DB_CONN.set(Mutex::new(conn)).map_err(|_| "Database connection already initialized".to_string())?;

    tracing::info!("SQLite bug reports database initialized at {db_path}");
    Ok(())
}

/// Get the database connection, initializing if necessary.
#[cfg(feature = "ssr")]
fn get_conn() -> Result<std::sync::MutexGuard<'static, Connection>, String> {
    if DB_CONN.get().is_none() {
        init_db()?;
    }
    DB_CONN
        .get()
        .ok_or_else(|| "Database connection not initialized".to_string())?
        .lock()
        .map_err(|err| format!("Failed to lock database connection: {err}"))
}

/// Compute a similarity hash from message, exception_message, and stack_trace.
/// Used to group identical bug reports together.
#[cfg(feature = "ssr")]
fn compute_similarity_hash(message: &str, exception_message: &Option<String>, stack_trace: &Option<String>) -> i64 {
    let mut hasher = DefaultHasher::new();
    message.hash(&mut hasher);
    exception_message.hash(&mut hasher);
    stack_trace.hash(&mut hasher);
    hasher.finish() as i64
}

/// Save a bug report to SQLite.
#[cfg(feature = "ssr")]
pub fn save_bug_report(report: &BugReportRequest) -> Result<i64, String> {
    let conn = get_conn()?;

    let bug_type_str: &'static str = report.bug_type.into();
    let similarity_hash = compute_similarity_hash(&report.message, &report.exception_message, &report.stack_trace);

    conn.execute(
        "INSERT INTO bug_reports (bug_type, similarity_hash, message, exception_message, stack_trace, user_login, url, user_agent, application)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        rusqlite::params![
            bug_type_str,
            similarity_hash,
            &report.message,
            &report.exception_message,
            &report.stack_trace,
            &report.user_login,
            &report.url,
            &report.user_agent,
            &report.application,
        ],
    )
    .map_err(|err| format!("Failed to insert bug report: {err}"))?;

    Ok(conn.last_insert_rowid())
}

/// A stored bug report retrieved from the database.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StoredBugReport {
    pub id: i64,
    pub bug_type: String,
    pub similarity_hash: i64,
    pub similar_count: i64,
    pub message: String,
    pub exception_message: Option<String>,
    pub stack_trace: Option<String>,
    pub user_login: Option<String>,
    pub url: Option<String>,
    pub user_agent: Option<String>,
    pub application: String,
    pub created_at: String,
}

/// Fetch all bug reports from SQLite, grouped by similarity_hash with counts.
/// Returns one representative bug per unique similarity_hash, ordered by most recent first.
#[cfg(feature = "ssr")]
pub fn fetch_all_bug_reports(limit: i64) -> Result<Vec<StoredBugReport>, String> {
    let conn = get_conn()?;

    // Get unique bugs (one per similarity_hash) with their counts
    // Uses a subquery to get the most recent bug per hash and count all similar bugs
    let mut stmt = conn
        .prepare(
            "WITH unique_bugs AS (
                SELECT
                    id, bug_type, similarity_hash, message, exception_message,
                    stack_trace, user_login, url, user_agent, application, created_at,
                    ROW_NUMBER() OVER (PARTITION BY similarity_hash ORDER BY id DESC) as rn
                FROM bug_reports
            )
            SELECT
                u.id, u.bug_type, u.similarity_hash, u.message, u.exception_message,
                u.stack_trace, u.user_login, u.url, u.user_agent, u.application, u.created_at,
                (SELECT COUNT(*) FROM bug_reports b2 WHERE b2.similarity_hash = u.similarity_hash) as similar_count
            FROM unique_bugs u
            WHERE u.rn = 1
            ORDER BY u.id DESC
            LIMIT ?1",
        )
        .map_err(|err| format!("Failed to prepare statement: {err}"))?;

    let reports = stmt
        .query_map([limit], |row| {
            Ok(StoredBugReport {
                id: row.get(0)?,
                bug_type: row.get(1)?,
                similarity_hash: row.get(2)?,
                message: row.get(3)?,
                exception_message: row.get(4)?,
                stack_trace: row.get(5)?,
                user_login: row.get(6)?,
                url: row.get(7)?,
                user_agent: row.get(8)?,
                application: row.get(9)?,
                created_at: row.get(10)?,
                similar_count: row.get(11)?,
            })
        })
        .map_err(|err| format!("Failed to query bug reports: {err}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| format!("Failed to collect bug reports: {err}"))?;

    Ok(reports)
}

/// Delete all bug reports with the given similarity_hash.
#[cfg(feature = "ssr")]
pub fn delete_bug_reports_by_hash(similarity_hash: i64) -> Result<usize, String> {
    let conn = get_conn()?;

    let count = conn
        .execute("DELETE FROM bug_reports WHERE similarity_hash = ?1", [similarity_hash])
        .map_err(|err| format!("Failed to delete bug reports: {err}"))?;

    Ok(count)
}

/// Delete all bug reports.
#[cfg(feature = "ssr")]
pub fn delete_all_bug_reports() -> Result<usize, String> {
    let conn = get_conn()?;

    let count = conn
        .execute("DELETE FROM bug_reports", [])
        .map_err(|err| format!("Failed to delete all bug reports: {err}"))?;

    Ok(count)
}

/// Get count of bug reports.
#[cfg(feature = "ssr")]
pub fn count_bug_reports() -> Result<i64, String> {
    let conn = get_conn()?;

    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM bug_reports", [], |row| row.get(0))
        .map_err(|err| format!("Failed to count bug reports: {err}"))?;

    Ok(count)
}

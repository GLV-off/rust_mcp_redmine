use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for listing time entries with optional filters.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListTimeEntriesParams {
    /// Filter by project ID or project identifier.
    pub project_id: Option<String>,
    /// Filter by date (`YYYY-MM-DD`).
    pub spent_on: Option<String>,
    /// Maximum results per page (default 25, max 100).
    pub limit: Option<u64>,
    /// Offset for pagination.
    pub offset: Option<u64>,
}

/// Parameters for creating a new time entry.
///
/// `hours` is required; either `issue_id` or `project_id` must be provided.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateTimeEntryParams {
    /// Issue ID to log time against.
    pub issue_id: Option<u64>,
    /// Project ID (required if `issue_id` is not provided).
    pub project_id: Option<u64>,
    /// Number of hours spent (required).
    pub hours: f64,
    /// Activity ID.
    pub activity_id: Option<u64>,
    /// Comments for the time entry.
    pub comments: Option<String>,
    /// Date (`YYYY-MM-DD`, defaults to today).
    pub spent_on: Option<String>,
}

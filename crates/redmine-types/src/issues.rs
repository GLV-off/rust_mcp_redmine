use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for listing issues with optional filters.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListIssuesParams {
    /// Filter by project ID.
    pub project_id: Option<u64>,
    /// Status filter: `"open"`, `"closed"`, `"all"`, or a numeric status ID.
    pub status_id: Option<String>,
    /// Filter by tracker ID.
    pub tracker_id: Option<u64>,
    /// Filter by assignee user ID.
    pub assignee_id: Option<u64>,
    /// Maximum results per page (default 25, max 100).
    pub limit: Option<u64>,
    /// Offset for pagination.
    pub offset: Option<u64>,
}

/// Parameters for getting a single issue by ID.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetIssueParams {
    /// Issue ID.
    pub id: u64,
}

/// Parameters for creating a new issue.
///
/// Only `project_id` and `subject` are required; all other fields are optional.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateIssueParams {
    /// Project ID to create the issue in (required).
    pub project_id: u64,
    /// Tracker ID.
    pub tracker_id: Option<u64>,
    /// Status ID.
    pub status_id: Option<u64>,
    /// Priority ID.
    pub priority_id: Option<u64>,
    /// Issue subject (required).
    pub subject: String,
    /// Issue description.
    pub description: Option<String>,
    /// User ID to assign the issue to.
    pub assigned_to_id: Option<u64>,
    /// Parent issue ID for subtasks.
    pub parent_issue_id: Option<u64>,
    /// Estimated hours.
    pub estimated_hours: Option<f64>,
}

/// Parameters for updating an existing issue.
///
/// All fields except `id` are optional — only provided fields will be updated.
/// Use `notes` to add a comment to the update.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateIssueParams {
    /// Issue ID to update (required).
    pub id: u64,
    /// New project ID.
    pub project_id: Option<u64>,
    /// New tracker ID.
    pub tracker_id: Option<u64>,
    /// New status ID.
    pub status_id: Option<u64>,
    /// New priority ID.
    pub priority_id: Option<u64>,
    /// New subject.
    pub subject: Option<String>,
    /// New description.
    pub description: Option<String>,
    /// New assignee user ID.
    pub assigned_to_id: Option<u64>,
    /// New parent issue ID.
    pub parent_issue_id: Option<u64>,
    /// New estimated hours.
    pub estimated_hours: Option<f64>,
    /// Notes to add to the issue update.
    pub notes: Option<String>,
}

/// Parameters for deleting an issue by ID.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeleteIssueParams {
    /// Issue ID to delete.
    pub id: u64,
}

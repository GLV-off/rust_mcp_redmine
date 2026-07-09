use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListIssuesParams {
    pub project_id: Option<u64>,
    pub status_id: Option<String>,
    pub tracker_id: Option<u64>,
    pub assignee_id: Option<u64>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetIssueParams {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateIssueParams {
    pub project_id: u64,
    pub tracker_id: Option<u64>,
    pub status_id: Option<u64>,
    pub priority_id: Option<u64>,
    pub subject: String,
    pub description: Option<String>,
    pub assigned_to_id: Option<u64>,
    pub parent_issue_id: Option<u64>,
    pub estimated_hours: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateIssueParams {
    pub id: u64,
    pub project_id: Option<u64>,
    pub tracker_id: Option<u64>,
    pub status_id: Option<u64>,
    pub priority_id: Option<u64>,
    pub subject: Option<String>,
    pub description: Option<String>,
    pub assigned_to_id: Option<u64>,
    pub parent_issue_id: Option<u64>,
    pub estimated_hours: Option<f64>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeleteIssueParams {
    pub id: u64,
}

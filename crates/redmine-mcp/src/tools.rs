use std::sync::Arc;

use rmcp::{
    handler::server::wrapper::Parameters,
    schemars::JsonSchema,
    tool,
    tool_router,
    ErrorData as McpError,
};
use serde::Deserialize;
use serde_json::json;

use redmine_core::client::RedmineClient;

/// MCP server that exposes Redmine operations as tools.
///
/// Each method annotated with `#[tool(..)]` becomes an MCP tool callable
/// by any MCP-compatible client (e.g. AI assistants).
#[derive(Clone)]
pub struct RedmineServer {
    client: Arc<RedmineClient>,
}

impl RedmineServer {
    /// Create a new server wrapping the given [`RedmineClient`].
    pub fn new(client: RedmineClient) -> Self {
        Self {
            client: Arc::new(client),
        }
    }
}

// ── Issues ──

/// Arguments for the `redmine_list_issues` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListIssuesArgs {
    #[schemars(description = "Project ID to filter by")]
    project_id: Option<u64>,
    #[schemars(description = "Status filter: open, closed, all, or a specific status ID number")]
    status_id: Option<String>,
    #[schemars(description = "Tracker ID to filter by")]
    tracker_id: Option<u64>,
    #[schemars(description = "Assignee user ID to filter by")]
    assignee_id: Option<u64>,
    #[schemars(description = "Maximum results per page (max 100, default 25)")]
    limit: Option<u64>,
    #[schemars(description = "Offset for pagination")]
    offset: Option<u64>,
}

/// Arguments for the `redmine_get_issue` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetIssueArgs {
    #[schemars(description = "Issue ID")]
    id: u64,
}

/// Arguments for the `redmine_create_issue` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateIssueArgs {
    #[schemars(description = "Project ID")]
    project_id: u64,
    #[schemars(description = "Issue subject")]
    subject: String,
    #[schemars(description = "Issue description")]
    description: Option<String>,
    #[schemars(description = "Tracker ID")]
    tracker_id: Option<u64>,
    #[schemars(description = "Status ID")]
    status_id: Option<u64>,
    #[schemars(description = "Priority ID")]
    priority_id: Option<u64>,
    #[schemars(description = "Assignee user ID")]
    assigned_to_id: Option<u64>,
    #[schemars(description = "Parent issue ID")]
    parent_issue_id: Option<u64>,
    #[schemars(description = "Estimated hours")]
    estimated_hours: Option<f64>,
}

/// Arguments for the `redmine_update_issue` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateIssueArgs {
    #[schemars(description = "Issue ID")]
    id: u64,
    #[schemars(description = "New project ID")]
    project_id: Option<u64>,
    #[schemars(description = "New tracker ID")]
    tracker_id: Option<u64>,
    #[schemars(description = "New subject")]
    subject: Option<String>,
    #[schemars(description = "New description")]
    description: Option<String>,
    #[schemars(description = "New status ID")]
    status_id: Option<u64>,
    #[schemars(description = "New priority ID")]
    priority_id: Option<u64>,
    #[schemars(description = "New assignee user ID")]
    assigned_to_id: Option<u64>,
    #[schemars(description = "New parent issue ID")]
    parent_issue_id: Option<u64>,
    #[schemars(description = "New estimated hours")]
    estimated_hours: Option<f64>,
    #[schemars(description = "Notes to add to the issue")]
    notes: Option<String>,
}

/// Arguments for the `redmine_delete_issue` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteIssueArgs {
    #[schemars(description = "Issue ID to delete")]
    id: u64,
}

// ── Projects ──

/// Arguments for the `redmine_list_projects` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListProjectsArgs {
    #[schemars(description = "Maximum results per page (max 100, default 25)")]
    limit: Option<u64>,
    #[schemars(description = "Offset for pagination")]
    offset: Option<u64>,
}

/// Arguments for the `redmine_get_project` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetProjectArgs {
    #[schemars(description = "Project ID")]
    id: u64,
}

// ── Users ──

/// Arguments for the `redmine_list_users` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListUsersArgs {
    #[schemars(description = "Maximum results per page (max 100, default 25)")]
    limit: Option<u64>,
    #[schemars(description = "Offset for pagination")]
    offset: Option<u64>,
}

/// Arguments for the `redmine_get_user` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetUserArgs {
    #[schemars(description = "User ID")]
    id: u64,
}

// ── Time Entries ──

/// Arguments for the `redmine_list_time_entries` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListTimeEntriesArgs {
    #[schemars(description = "Filter by project ID or identifier")]
    project_id: Option<String>,
    #[schemars(description = "Filter by date (YYYY-MM-DD)")]
    spent_on: Option<String>,
    #[schemars(description = "Maximum results per page (max 100, default 25)")]
    limit: Option<u64>,
    #[schemars(description = "Offset for pagination")]
    offset: Option<u64>,
}

/// Arguments for the `redmine_create_time_entry` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateTimeEntryArgs {
    #[schemars(description = "Issue ID to log time against")]
    issue_id: Option<u64>,
    #[schemars(description = "Project ID (required if no issue_id)")]
    project_id: Option<u64>,
    #[schemars(description = "Number of hours spent")]
    hours: f64,
    #[schemars(description = "Activity ID")]
    activity_id: Option<u64>,
    #[schemars(description = "Comments for the time entry")]
    comments: Option<String>,
    #[schemars(description = "Date (YYYY-MM-DD, defaults to today)")]
    spent_on: Option<String>,
}

#[tool_router(server_handler)]
impl RedmineServer {
    #[tool(description = "List issues from Redmine with optional filters")]
    async fn redmine_list_issues(
        &self,
        Parameters(args): Parameters<ListIssuesArgs>,
    ) -> Result<String, McpError> {
        self.client
            .list_issues(
                args.project_id,
                args.status_id,
                args.tracker_id,
                args.assignee_id,
                args.limit.unwrap_or(25),
                args.offset.unwrap_or(0),
            )
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))
    }

    #[tool(description = "Get a single issue by ID")]
    async fn redmine_get_issue(
        &self,
        Parameters(args): Parameters<GetIssueArgs>,
    ) -> Result<String, McpError> {
        self.client
            .get_issue(args.id)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))
    }

    #[tool(description = "Create a new issue in Redmine")]
    async fn redmine_create_issue(
        &self,
        Parameters(args): Parameters<CreateIssueArgs>,
    ) -> Result<String, McpError> {
        self.client
            .create_issue(
                args.project_id,
                &args.subject,
                args.description.as_deref(),
                args.tracker_id,
                args.status_id,
                args.priority_id,
                args.assigned_to_id,
                args.parent_issue_id,
                args.estimated_hours,
            )
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))
    }

    #[tool(description = "Update an existing issue in Redmine")]
    async fn redmine_update_issue(
        &self,
        Parameters(args): Parameters<UpdateIssueArgs>,
    ) -> Result<String, McpError> {
        self.client
            .update_issue(
                args.id,
                args.project_id,
                args.tracker_id,
                args.subject.as_deref(),
                args.description.as_deref(),
                args.status_id,
                args.priority_id,
                args.assigned_to_id,
                args.parent_issue_id,
                args.estimated_hours,
                args.notes.as_deref(),
            )
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(json!({ "status": "updated", "id": args.id }).to_string())
    }

    #[tool(description = "Delete an issue from Redmine")]
    async fn redmine_delete_issue(
        &self,
        Parameters(args): Parameters<DeleteIssueArgs>,
    ) -> Result<String, McpError> {
        self.client
            .delete_issue(args.id)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(json!({ "status": "deleted", "id": args.id }).to_string())
    }

    #[tool(description = "List all projects")]
    async fn redmine_list_projects(
        &self,
        Parameters(args): Parameters<ListProjectsArgs>,
    ) -> Result<String, McpError> {
        self.client
            .list_projects(args.limit.unwrap_or(25), args.offset.unwrap_or(0))
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))
    }

    #[tool(description = "Get a single project by ID")]
    async fn redmine_get_project(
        &self,
        Parameters(args): Parameters<GetProjectArgs>,
    ) -> Result<String, McpError> {
        self.client
            .get_project(args.id)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))
    }

    #[tool(description = "List all users")]
    async fn redmine_list_users(
        &self,
        Parameters(args): Parameters<ListUsersArgs>,
    ) -> Result<String, McpError> {
        self.client
            .list_users(args.limit.unwrap_or(25), args.offset.unwrap_or(0))
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))
    }

    #[tool(description = "Get a single user by ID")]
    async fn redmine_get_user(
        &self,
        Parameters(args): Parameters<GetUserArgs>,
    ) -> Result<String, McpError> {
        self.client
            .get_user(args.id)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))
    }

    #[tool(description = "List time entries with optional filters")]
    async fn redmine_list_time_entries(
        &self,
        Parameters(args): Parameters<ListTimeEntriesArgs>,
    ) -> Result<String, McpError> {
        self.client
            .list_time_entries(
                args.project_id,
                args.spent_on,
                args.limit.unwrap_or(25),
                args.offset.unwrap_or(0),
            )
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))
    }

    #[tool(description = "Create a new time entry")]
    async fn redmine_create_time_entry(
        &self,
        Parameters(args): Parameters<CreateTimeEntryArgs>,
    ) -> Result<String, McpError> {
        self.client
            .create_time_entry(
                args.issue_id,
                args.project_id,
                args.hours,
                args.activity_id,
                args.comments.as_deref(),
                args.spent_on.as_deref(),
            )
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_issues_args_defaults() {
        let json = serde_json::json!({});
        let args: ListIssuesArgs = serde_json::from_value(json).unwrap();
        assert!(args.project_id.is_none());
        assert!(args.status_id.is_none());
        assert!(args.tracker_id.is_none());
        assert!(args.assignee_id.is_none());
        assert!(args.limit.is_none());
        assert!(args.offset.is_none());
    }

    #[test]
    fn test_list_issues_args_full() {
        let json = serde_json::json!({
            "project_id": 1,
            "status_id": "open",
            "tracker_id": 2,
            "assignee_id": 3,
            "limit": 50,
            "offset": 10
        });
        let args: ListIssuesArgs = serde_json::from_value(json).unwrap();
        assert_eq!(args.project_id, Some(1));
        assert_eq!(args.status_id, Some("open".into()));
        assert_eq!(args.tracker_id, Some(2));
        assert_eq!(args.assignee_id, Some(3));
        assert_eq!(args.limit, Some(50));
        assert_eq!(args.offset, Some(10));
    }

    #[test]
    fn test_get_issue_args() {
        let json = serde_json::json!({ "id": 42 });
        let args: GetIssueArgs = serde_json::from_value(json).unwrap();
        assert_eq!(args.id, 42);
    }

    #[test]
    fn test_create_issue_args_full() {
        let json = serde_json::json!({
            "project_id": 1,
            "subject": "Test",
            "description": "Desc",
            "tracker_id": 2,
            "status_id": 3,
            "priority_id": 4,
            "assigned_to_id": 5,
            "parent_issue_id": 6,
            "estimated_hours": 3.5
        });
        let args: CreateIssueArgs = serde_json::from_value(json).unwrap();
        assert_eq!(args.project_id, 1);
        assert_eq!(args.subject, "Test");
        assert_eq!(args.description, Some("Desc".into()));
        assert_eq!(args.tracker_id, Some(2));
        assert_eq!(args.status_id, Some(3));
        assert_eq!(args.priority_id, Some(4));
        assert_eq!(args.assigned_to_id, Some(5));
        assert_eq!(args.parent_issue_id, Some(6));
        assert_eq!(args.estimated_hours, Some(3.5));
    }

    #[test]
    fn test_create_issue_args_minimal() {
        let json = serde_json::json!({
            "project_id": 1,
            "subject": "Minimal"
        });
        let args: CreateIssueArgs = serde_json::from_value(json).unwrap();
        assert_eq!(args.project_id, 1);
        assert_eq!(args.subject, "Minimal");
        assert!(args.description.is_none());
        assert!(args.tracker_id.is_none());
    }

    #[test]
    fn test_update_issue_args_minimal() {
        let json = serde_json::json!({
            "id": 42,
            "subject": "Updated",
            "notes": "Fixed"
        });
        let args: UpdateIssueArgs = serde_json::from_value(json).unwrap();
        assert_eq!(args.id, 42);
        assert_eq!(args.subject, Some("Updated".into()));
        assert_eq!(args.notes, Some("Fixed".into()));
        assert!(args.description.is_none());
        assert!(args.status_id.is_none());
        assert!(args.project_id.is_none());
        assert!(args.tracker_id.is_none());
        assert!(args.parent_issue_id.is_none());
    }

    #[test]
    fn test_update_issue_args_full() {
        let json = serde_json::json!({
            "id": 42,
            "project_id": 1,
            "tracker_id": 2,
            "subject": "Updated",
            "description": "New desc",
            "status_id": 3,
            "priority_id": 4,
            "assigned_to_id": 5,
            "parent_issue_id": 6,
            "estimated_hours": 3.5,
            "notes": "Fixed"
        });
        let args: UpdateIssueArgs = serde_json::from_value(json).unwrap();
        assert_eq!(args.id, 42);
        assert_eq!(args.project_id, Some(1));
        assert_eq!(args.tracker_id, Some(2));
        assert_eq!(args.subject, Some("Updated".into()));
        assert_eq!(args.description, Some("New desc".into()));
        assert_eq!(args.status_id, Some(3));
        assert_eq!(args.priority_id, Some(4));
        assert_eq!(args.assigned_to_id, Some(5));
        assert_eq!(args.parent_issue_id, Some(6));
        assert_eq!(args.estimated_hours, Some(3.5));
        assert_eq!(args.notes, Some("Fixed".into()));
    }

    #[test]
    fn test_delete_issue_args() {
        let json = serde_json::json!({ "id": 99 });
        let args: DeleteIssueArgs = serde_json::from_value(json).unwrap();
        assert_eq!(args.id, 99);
    }

    #[test]
    fn test_list_projects_args() {
        let json = serde_json::json!({ "limit": 10, "offset": 5 });
        let args: ListProjectsArgs = serde_json::from_value(json).unwrap();
        assert_eq!(args.limit, Some(10));
        assert_eq!(args.offset, Some(5));
    }

    #[test]
    fn test_list_projects_args_defaults() {
        let json = serde_json::json!({});
        let args: ListProjectsArgs = serde_json::from_value(json).unwrap();
        assert!(args.limit.is_none());
        assert!(args.offset.is_none());
    }

    #[test]
    fn test_get_project_args() {
        let json = serde_json::json!({ "id": 7 });
        let args: GetProjectArgs = serde_json::from_value(json).unwrap();
        assert_eq!(args.id, 7);
    }

    #[test]
    fn test_list_users_args() {
        let json = serde_json::json!({ "limit": 100 });
        let args: ListUsersArgs = serde_json::from_value(json).unwrap();
        assert_eq!(args.limit, Some(100));
        assert!(args.offset.is_none());
    }

    #[test]
    fn test_get_user_args() {
        let json = serde_json::json!({ "id": 3 });
        let args: GetUserArgs = serde_json::from_value(json).unwrap();
        assert_eq!(args.id, 3);
    }

    #[test]
    fn test_list_time_entries_args() {
        let json = serde_json::json!({
            "project_id": "my-project",
            "spent_on": "2024-01-15",
            "limit": 50
        });
        let args: ListTimeEntriesArgs = serde_json::from_value(json).unwrap();
        assert_eq!(args.project_id, Some("my-project".into()));
        assert_eq!(args.spent_on, Some("2024-01-15".into()));
        assert_eq!(args.limit, Some(50));
        assert!(args.offset.is_none());
    }

    #[test]
    fn test_list_time_entries_args_defaults() {
        let json = serde_json::json!({});
        let args: ListTimeEntriesArgs = serde_json::from_value(json).unwrap();
        assert!(args.project_id.is_none());
        assert!(args.spent_on.is_none());
        assert!(args.limit.is_none());
        assert!(args.offset.is_none());
    }

    #[test]
    fn test_create_time_entry_args_full() {
        let json = serde_json::json!({
            "issue_id": 1,
            "project_id": null,
            "hours": 2.5,
            "activity_id": 9,
            "comments": "Implementation",
            "spent_on": "2024-01-15"
        });
        let args: CreateTimeEntryArgs = serde_json::from_value(json).unwrap();
        assert_eq!(args.issue_id, Some(1));
        assert!(args.project_id.is_none());
        assert_eq!(args.hours, 2.5);
        assert_eq!(args.activity_id, Some(9));
        assert_eq!(args.comments, Some("Implementation".into()));
        assert_eq!(args.spent_on, Some("2024-01-15".into()));
    }

    #[test]
    fn test_create_time_entry_args_minimal() {
        let json = serde_json::json!({
            "project_id": 1,
            "hours": 1.0
        });
        let args: CreateTimeEntryArgs = serde_json::from_value(json).unwrap();
        assert!(args.issue_id.is_none());
        assert_eq!(args.project_id, Some(1));
        assert_eq!(args.hours, 1.0);
        assert!(args.comments.is_none());
    }
}

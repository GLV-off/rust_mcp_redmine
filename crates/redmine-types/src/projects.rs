use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for listing projects with pagination.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListProjectsParams {
    /// Maximum results per page (default 25, max 100).
    pub limit: Option<u64>,
    /// Offset for pagination.
    pub offset: Option<u64>,
}

/// Parameters for getting a single project by ID.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetProjectParams {
    /// Project ID.
    pub id: u64,
}

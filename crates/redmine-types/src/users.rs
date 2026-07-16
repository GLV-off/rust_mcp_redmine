use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for listing users with pagination.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListUsersParams {
    /// Maximum results per page (default 25, max 100).
    pub limit: Option<u64>,
    /// Offset for pagination.
    pub offset: Option<u64>,
}

/// Parameters for getting a single user by ID.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetUserParams {
    /// User ID.
    pub id: u64,
}

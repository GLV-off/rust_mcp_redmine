use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListTimeEntriesParams {
    pub project_id: Option<String>,
    pub spent_on: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateTimeEntryParams {
    pub issue_id: Option<u64>,
    pub project_id: Option<u64>,
    pub hours: f64,
    pub activity_id: Option<u64>,
    pub comments: Option<String>,
    pub spent_on: Option<String>,
}

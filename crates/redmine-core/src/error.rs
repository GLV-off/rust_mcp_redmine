use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Redmine API error: {0}")]
    RedmineApi(#[from] redmine_api::Error),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Builder error: {0}")]
    Builder(String),

    #[error("Time parse error: {0}")]
    TimeParse(String, #[source] time::error::Parse),
}

impl From<redmine_api::api::issues::ListIssuesBuilderError> for CoreError {
    fn from(e: redmine_api::api::issues::ListIssuesBuilderError) -> Self {
        CoreError::Builder(e.to_string())
    }
}

impl From<redmine_api::api::issues::GetIssueBuilderError> for CoreError {
    fn from(e: redmine_api::api::issues::GetIssueBuilderError) -> Self {
        CoreError::Builder(e.to_string())
    }
}

impl From<redmine_api::api::issues::CreateIssueBuilderError> for CoreError {
    fn from(e: redmine_api::api::issues::CreateIssueBuilderError) -> Self {
        CoreError::Builder(e.to_string())
    }
}

impl From<redmine_api::api::issues::UpdateIssueBuilderError> for CoreError {
    fn from(e: redmine_api::api::issues::UpdateIssueBuilderError) -> Self {
        CoreError::Builder(e.to_string())
    }
}

impl From<redmine_api::api::issues::DeleteIssueBuilderError> for CoreError {
    fn from(e: redmine_api::api::issues::DeleteIssueBuilderError) -> Self {
        CoreError::Builder(e.to_string())
    }
}

impl From<redmine_api::api::projects::ListProjectsBuilderError> for CoreError {
    fn from(e: redmine_api::api::projects::ListProjectsBuilderError) -> Self {
        CoreError::Builder(e.to_string())
    }
}

impl From<redmine_api::api::projects::GetProjectBuilderError> for CoreError {
    fn from(e: redmine_api::api::projects::GetProjectBuilderError) -> Self {
        CoreError::Builder(e.to_string())
    }
}

impl From<redmine_api::api::users::ListUsersBuilderError> for CoreError {
    fn from(e: redmine_api::api::users::ListUsersBuilderError) -> Self {
        CoreError::Builder(e.to_string())
    }
}

impl From<redmine_api::api::users::GetUserBuilderError> for CoreError {
    fn from(e: redmine_api::api::users::GetUserBuilderError) -> Self {
        CoreError::Builder(e.to_string())
    }
}

impl From<redmine_api::api::time_entries::ListTimeEntriesBuilderError> for CoreError {
    fn from(e: redmine_api::api::time_entries::ListTimeEntriesBuilderError) -> Self {
        CoreError::Builder(e.to_string())
    }
}

impl From<redmine_api::api::time_entries::CreateTimeEntryBuilderError> for CoreError {
    fn from(e: redmine_api::api::time_entries::CreateTimeEntryBuilderError) -> Self {
        CoreError::Builder(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_error_display() {
        let err = CoreError::Config("missing field".into());
        assert_eq!(err.to_string(), "Config error: missing field");

        let err = CoreError::Builder("build failed".into());
        assert_eq!(err.to_string(), "Builder error: build failed");
    }

    #[test]
    fn test_core_error_debug() {
        let err = CoreError::Config("x".into());
        assert!(format!("{err:?}").contains("Config"));
    }

    #[test]
    fn test_core_error_conversions() {
        let json_err = serde_json::from_str::<()>("invalid").unwrap_err();
        let converted: CoreError = json_err.into();
        assert!(matches!(converted, CoreError::Json(_)));

        let url_err = url::Url::parse("not a url").unwrap_err();
        let converted: CoreError = url_err.into();
        assert!(matches!(converted, CoreError::Url(_)));
    }
}

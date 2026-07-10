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
    use std::error::Error;

    #[test]
    fn test_core_error_display() {
        let err = CoreError::Config("missing field".into());
        assert_eq!(err.to_string(), "Config error: missing field");

        let err = CoreError::Builder("build failed".into());
        assert_eq!(err.to_string(), "Builder error: build failed");

        let err = CoreError::RedmineApi(redmine_api::Error::HttpErrorResponse(reqwest::StatusCode::NOT_FOUND));
        assert!(err.to_string().contains("Redmine API error"));

        // Create a TimeParse error using a real parse failure
        let parse_result = time::Date::parse("2024-13-01", &time::macros::format_description!("[year]-[month]-[day]"));
        assert!(parse_result.is_err());
        let err = CoreError::TimeParse("2024-13-01".into(), parse_result.unwrap_err());
        assert!(err.to_string().contains("Time parse error"));
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

    #[test]
    fn test_builder_error_conversion() {
        // Test that builder errors from various endpoint types convert to CoreError::Builder
        use redmine_api::api::issues::ListIssuesBuilderError;
        let err = ListIssuesBuilderError::ValidationError("test".into());
        let converted: CoreError = err.into();
        assert!(matches!(converted, CoreError::Builder(_)));
        assert!(converted.to_string().contains("Builder error"));
    }

    #[test]
    fn test_core_error_source() {
        let inner = redmine_api::Error::HttpErrorResponse(reqwest::StatusCode::NOT_FOUND);
        let err = CoreError::RedmineApi(inner);
        let source = err.source();
        assert!(source.is_some(), "CoreError should have a source");
    }

    #[test]
    fn test_core_error_send_sync() {
        // CoreError must be Send + Sync for use in async contexts
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        assert_send::<CoreError>();
        assert_sync::<CoreError>();
    }
}

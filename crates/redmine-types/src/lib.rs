pub mod issues;
pub mod projects;
pub mod users;
pub mod time_entries;

#[cfg(test)]
mod tests {
    use crate::issues::*;
    use crate::projects::*;
    use crate::users::*;
    use crate::time_entries::*;

    #[test]
    fn test_list_issues_params_roundtrip() {
        let params = ListIssuesParams {
            project_id: Some(1),
            status_id: Some("open".into()),
            tracker_id: Some(2),
            assignee_id: Some(3),
            limit: Some(50),
            offset: Some(10),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["project_id"], 1);
        assert_eq!(json["status_id"], "open");
        assert_eq!(json["tracker_id"], 2);
        assert_eq!(json["assignee_id"], 3);
        assert_eq!(json["limit"], 50);
        assert_eq!(json["offset"], 10);

        let deserialized: ListIssuesParams = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.project_id, Some(1));
        assert_eq!(deserialized.status_id, Some("open".into()));
    }

    #[test]
    fn test_list_issues_params_defaults() {
        let params = ListIssuesParams {
            project_id: None,
            status_id: None,
            tracker_id: None,
            assignee_id: None,
            limit: None,
            offset: None,
        };
        let json = serde_json::to_value(&params).unwrap();
        assert!(json["project_id"].is_null());
        assert!(json["limit"].is_null());

        let deserialized: ListIssuesParams = serde_json::from_value(json).unwrap();
        assert!(deserialized.project_id.is_none());
    }

    #[test]
    fn test_create_issue_params_full() {
        let params = CreateIssueParams {
            project_id: 1,
            tracker_id: Some(2),
            status_id: Some(3),
            priority_id: Some(4),
            subject: "Test issue".into(),
            description: Some("A description".into()),
            assigned_to_id: Some(5),
            parent_issue_id: Some(6),
            estimated_hours: Some(3.5),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["project_id"], 1);
        assert_eq!(json["tracker_id"], 2);
        assert_eq!(json["status_id"], 3);
        assert_eq!(json["priority_id"], 4);
        assert_eq!(json["subject"], "Test issue");
        assert_eq!(json["description"], "A description");
        assert_eq!(json["assigned_to_id"], 5);
        assert_eq!(json["parent_issue_id"], 6);
        assert_eq!(json["estimated_hours"], 3.5);
    }

    #[test]
    fn test_create_issue_params_minimal() {
        let params = CreateIssueParams {
            project_id: 1,
            subject: "Minimal".into(),
            tracker_id: None,
            status_id: None,
            priority_id: None,
            description: None,
            assigned_to_id: None,
            parent_issue_id: None,
            estimated_hours: None,
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["project_id"], 1);
        assert_eq!(json["subject"], "Minimal");
        assert!(json["tracker_id"].is_null());
        assert!(json["description"].is_null());
    }

    #[test]
    fn test_update_issue_params() {
        let params = UpdateIssueParams {
            id: 42,
            project_id: Some(1),
            tracker_id: Some(2),
            status_id: Some(3),
            priority_id: Some(4),
            subject: Some("Updated".into()),
            description: Some("New desc".into()),
            assigned_to_id: Some(5),
            parent_issue_id: Some(6),
            estimated_hours: Some(2.0),
            notes: Some("Fixed".into()),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["id"], 42);
        assert_eq!(json["subject"], "Updated");
        assert_eq!(json["notes"], "Fixed");
        assert_eq!(json["estimated_hours"], 2.0);

        let deserialized: UpdateIssueParams = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.id, 42);
        assert_eq!(deserialized.notes, Some("Fixed".into()));
    }

    #[test]
    fn test_delete_issue_params() {
        let params = DeleteIssueParams { id: 99 };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["id"], 99);

        let deserialized: DeleteIssueParams = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.id, 99);
    }

    #[test]
    fn test_project_params() {
        let list = ListProjectsParams {
            limit: Some(10),
            offset: Some(0),
        };
        let json = serde_json::to_value(&list).unwrap();
        assert_eq!(json["limit"], 10);
        assert_eq!(json["offset"], 0);

        let get = GetProjectParams { id: 5 };
        let json = serde_json::to_value(&get).unwrap();
        assert_eq!(json["id"], 5);
    }

    #[test]
    fn test_user_params() {
        let list = ListUsersParams {
            limit: Some(25),
            offset: Some(0),
        };
        let json = serde_json::to_value(&list).unwrap();
        assert_eq!(json["limit"], 25);

        let get = GetUserParams { id: 3 };
        let json = serde_json::to_value(&get).unwrap();
        assert_eq!(json["id"], 3);
    }

    #[test]
    fn test_time_entry_params() {
        let list = ListTimeEntriesParams {
            project_id: Some("my-project".into()),
            spent_on: Some("2024-01-15".into()),
            limit: Some(10),
            offset: Some(0),
        };
        let json = serde_json::to_value(&list).unwrap();
        assert_eq!(json["project_id"], "my-project");
        assert_eq!(json["spent_on"], "2024-01-15");

        let create = CreateTimeEntryParams {
            issue_id: Some(1),
            project_id: None,
            hours: 2.5,
            activity_id: Some(9),
            comments: Some("Implementation".into()),
            spent_on: Some("2024-01-15".into()),
        };
        let json = serde_json::to_value(&create).unwrap();
        assert_eq!(json["issue_id"], 1);
        assert_eq!(json["hours"], 2.5);
        assert_eq!(json["activity_id"], 9);
        assert_eq!(json["comments"], "Implementation");
    }
}

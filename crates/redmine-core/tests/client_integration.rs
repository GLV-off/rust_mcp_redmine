use redmine_core::client::RedmineClient;
use redmine_core::config::Config;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn setup_mock() -> (MockServer, RedmineClient) {
    let mock_server = MockServer::start().await;
    let config = Config {
        redmine_url: mock_server.uri(),
        redmine_api_key: "test-api-key".into(),
    };
    let client = RedmineClient::new(config).unwrap();
    (mock_server, client)
}

fn dt() -> String {
    "2024-01-15T10:00:00Z".into()
}

fn issue_response(id: u64, subject: &str) -> serde_json::Value {
    json!({
        "id": id,
        "project": { "id": 1, "name": "Test Project" },
        "tracker": { "id": 1, "name": "Bug" },
        "status": { "id": 1, "name": "New", "is_closed": false },
        "priority": { "id": 2, "name": "Normal" },
        "author": { "id": 1, "name": "Test User" },
        "subject": subject,
        "description": null,
        "done_ratio": 0,
        "is_private": false,
        "start_date": null,
        "due_date": null,
        "closed_on": null,
        "estimated_hours": null,
        "parent": null,
        "assigned_to": null,
        "category": null,
        "fixed_version": null,
        "created_on": dt(),
        "updated_on": dt()
    })
}

#[tokio::test]
async fn test_list_issues() {
    let (mock, client) = setup_mock().await;

    Mock::given(method("GET"))
        .and(path("/issues.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "issues": [
                issue_response(1, "Issue One"),
                issue_response(2, "Issue Two"),
            ],
            "total_count": 2,
            "offset": 0,
            "limit": 25
        })))
        .mount(&mock)
        .await;

    let body = client.list_issues(None, None, None, None, 25, 0).await.unwrap();
    assert!(body.contains("Issue One"));
    assert!(body.contains("Issue Two"));
}

#[tokio::test]
async fn test_list_issues_with_filters() {
    let (mock, client) = setup_mock().await;

    Mock::given(method("GET"))
        .and(path("/issues.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "issues": [issue_response(1, "Filtered Issue")],
            "total_count": 1,
            "offset": 0,
            "limit": 10
        })))
        .mount(&mock)
        .await;

    client
        .list_issues(Some(1), Some("open".into()), Some(2), Some(3), 10, 0)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_issue() {
    let (mock, client) = setup_mock().await;

    Mock::given(method("GET"))
        .and(path("/issues/5.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "issue": issue_response(5, "Single Issue")
        })))
        .mount(&mock)
        .await;

    let body = client.get_issue(5).await.unwrap();
    assert!(body.contains("Single Issue"));
}

#[tokio::test]
async fn test_create_issue() {
    let (mock, client) = setup_mock().await;

    Mock::given(method("POST"))
        .and(path("/issues.json"))
        .and(header("content-type", "application/json"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "issue": issue_response(10, "New Issue")
        })))
        .mount(&mock)
        .await;

    let body = client
        .create_issue(1, "New Issue", Some("desc"), None, None, None, None, None, None)
        .await
        .unwrap();
    assert!(body.contains("New Issue"));
}

#[tokio::test]
async fn test_update_issue() {
    let (mock, client) = setup_mock().await;

    Mock::given(method("PUT"))
        .and(path("/issues/1.json"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock)
        .await;

    let result = client
        .update_issue(1, Some("Updated"), None, None, None, None, None, Some("note"))
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_issue() {
    let (mock, client) = setup_mock().await;

    Mock::given(method("DELETE"))
        .and(path("/issues/99.json"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock)
        .await;

    let result = client.delete_issue(99).await;
    assert!(result.is_ok());
}

fn project_response(id: u64, name: &str) -> serde_json::Value {
    json!({
        "id": id,
        "name": name,
        "identifier": name.to_lowercase().replace(' ', "_"),
        "description": null,
        "status": 1,
        "created_on": dt(),
        "updated_on": dt()
    })
}

#[tokio::test]
async fn test_list_projects() {
    let (mock, client) = setup_mock().await;

    Mock::given(method("GET"))
        .and(path("/projects.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "projects": [project_response(1, "Alpha"), project_response(2, "Beta")],
            "total_count": 2,
            "offset": 0,
            "limit": 25
        })))
        .mount(&mock)
        .await;

    let result = client.list_projects(25, 0).await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Alpha"));
}

#[tokio::test]
async fn test_get_project() {
    let (mock, client) = setup_mock().await;

    Mock::given(method("GET"))
        .and(path("/projects/3.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "project": project_response(3, "Gamma")
        })))
        .mount(&mock)
        .await;

    let result = client.get_project(3).await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Gamma"));
}

fn user_response(id: u64, login: &str) -> serde_json::Value {
    json!({
        "id": id,
        "login": login,
        "admin": false,
        "firstname": "Test",
        "lastname": "User",
        "mail": null,
        "created_on": dt(),
        "updated_on": dt(),
        "last_login_on": null,
        "passwd_changed_on": null
    })
}

#[tokio::test]
async fn test_list_users() {
    let (mock, client) = setup_mock().await;

    Mock::given(method("GET"))
        .and(path("/users.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "users": [user_response(1, "alice"), user_response(2, "bob")],
            "total_count": 2,
            "offset": 0,
            "limit": 25
        })))
        .mount(&mock)
        .await;

    let result = client.list_users(25, 0).await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("alice"));
}

#[tokio::test]
async fn test_get_user() {
    let (mock, client) = setup_mock().await;

    Mock::given(method("GET"))
        .and(path("/users/4.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "user": user_response(4, "charlie")
        })))
        .mount(&mock)
        .await;

    let result = client.get_user(4).await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("charlie"));
}

fn time_entry_response(id: u64, hours: f64) -> serde_json::Value {
    json!({
        "id": id,
        "user": { "id": 1, "name": "Test User" },
        "hours": hours,
        "activity": { "id": 9, "name": "Development" },
        "comments": null,
        "issue": { "id": 1, "name": "#1: New Issue" },
        "project": { "id": 1, "name": "Test Project" },
        "spent_on": "2024-01-15",
        "created_on": dt(),
        "updated_on": dt()
    })
}

#[tokio::test]
async fn test_list_time_entries() {
    let (mock, client) = setup_mock().await;

    Mock::given(method("GET"))
        .and(path("/time_entries.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "time_entries": [time_entry_response(1, 2.5), time_entry_response(2, 1.0)],
            "total_count": 2,
            "offset": 0,
            "limit": 25
        })))
        .mount(&mock)
        .await;

    let result = client.list_time_entries(None, None, 25, 0).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_time_entry() {
    let (mock, client) = setup_mock().await;

    Mock::given(method("POST"))
        .and(path("/time_entries.json"))
        .and(header("content-type", "application/json"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "time_entry": time_entry_response(5, 3.0)
        })))
        .mount(&mock)
        .await;

    let result = client
        .create_time_entry(Some(1), None, 3.0, Some(9), Some("work"), Some("2024-01-15"))
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_time_entry_minimal() {
    let (mock, client) = setup_mock().await;

    Mock::given(method("POST"))
        .and(path("/time_entries.json"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "time_entry": time_entry_response(6, 1.5)
        })))
        .mount(&mock)
        .await;

    let result = client
        .create_time_entry(None, Some(1), 1.5, None, None, None)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_auth_header_sent() {
    let (mock, client) = setup_mock().await;

    Mock::given(method("GET"))
        .and(path("/issues.json"))
        .and(header("x-redmine-api-key", "test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "issues": [],
            "total_count": 0,
            "offset": 0,
            "limit": 25
        })))
        .mount(&mock)
        .await;

    let result = client.list_issues(None, None, None, None, 25, 0).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_error_response() {
    let (mock, client) = setup_mock().await;

    Mock::given(method("GET"))
        .and(path("/issues/999.json"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock)
        .await;

    let result = client.get_issue(999).await;
    assert!(result.is_err());
}

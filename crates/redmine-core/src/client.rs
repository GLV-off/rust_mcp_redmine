use std::sync::Arc;

use redmine_api::api::issues::{
    CreateIssue, DeleteIssue, GetIssue, Issue, IssueWrapper, ListIssues, UpdateIssue,
};
use redmine_api::api::projects::{GetProject, ListProjects, Project, ProjectWrapper};
use redmine_api::api::time_entries::{
    CreateTimeEntry, ListTimeEntries, TimeEntry, TimeEntryWrapper,
};
use redmine_api::api::users::{GetUser, ListUsers, User, UserWrapper};
use redmine_api::api::{RedmineAsync, ResponsePage};
use url::Url;

use crate::config::Config;
use crate::error::CoreError;

#[derive(Clone)]
pub struct RedmineClient {
    inner: Arc<RedmineAsync>,
}

impl RedmineClient {
    pub fn new(config: Config) -> Result<Self, CoreError> {
        let client = redmine_api::reqwest::Client::builder()
            .user_agent("redmine-mcp/0.1.0")
            .build()?;
        let url = Url::parse(&config.redmine_url)?;
        let inner = RedmineAsync::new(client, url, &config.redmine_api_key)?;
        Ok(Self { inner })
    }

    pub fn from_env() -> Result<Self, CoreError> {
        let client = redmine_api::reqwest::Client::builder()
            .user_agent("redmine-mcp/0.1.0")
            .build()?;
        let inner = RedmineAsync::from_env(client)?;
        Ok(Self { inner })
    }

    pub async fn list_issues(
        &self,
        project_id: Option<u64>,
        status_id: Option<String>,
        tracker_id: Option<u64>,
        assignee_id: Option<u64>,
        limit: u64,
        offset: u64,
    ) -> Result<String, CoreError> {
        let mut builder = ListIssues::builder();
        if let Some(pid) = project_id {
            builder.project_id(vec![pid]);
        }
        if let Some(sid) = status_id {
            builder.status_id(parse_issue_status_filter(&sid));
        }
        if let Some(tid) = tracker_id {
            builder.tracker_id(vec![tid]);
        }
        if let Some(aid) = assignee_id {
            builder.assignee(redmine_api::api::issues::AssigneeFilter::TheseAssignees(vec![aid]));
        }

        let endpoint = builder.build()?;
        let ResponsePage {
            values: issues,
            total_count,
            ..
        } = self
            .inner
            .clone()
            .json_response_body_page::<_, Issue>(&endpoint, offset, limit)
            .await?;

        serde_json::to_string_pretty(&serde_json::json!({
            "total_count": total_count,
            "issues": issues,
        }))
        .map_err(CoreError::from)
    }

    pub async fn get_issue(&self, id: u64) -> Result<String, CoreError> {
        let endpoint = GetIssue::builder().id(id).build()?;
        let IssueWrapper { issue } = self
            .inner
            .clone()
            .json_response_body::<_, IssueWrapper<Issue>>(&endpoint)
            .await?;
        serde_json::to_string_pretty(&issue).map_err(CoreError::from)
    }

    pub async fn create_issue(
        &self,
        project_id: u64,
        subject: &str,
        description: Option<&str>,
        tracker_id: Option<u64>,
        status_id: Option<u64>,
        priority_id: Option<u64>,
        assigned_to_id: Option<u64>,
        parent_issue_id: Option<u64>,
        estimated_hours: Option<f64>,
    ) -> Result<String, CoreError> {
        let mut base = CreateIssue::builder();
        let builder = base
            .project_id(project_id)
            .subject(subject.to_owned());
        if let Some(desc) = description {
            builder.description(desc.to_owned());
        }
        if let Some(tid) = tracker_id {
            builder.tracker_id(tid);
        }
        if let Some(sid) = status_id {
            builder.status_id(sid);
        }
        if let Some(pid) = priority_id {
            builder.priority_id(pid);
        }
        if let Some(aid) = assigned_to_id {
            builder.assigned_to_id(aid);
        }
        if let Some(pid) = parent_issue_id {
            builder.parent_issue_id(pid);
        }
        if let Some(eh) = estimated_hours {
            builder.estimated_hours(eh);
        }

        let endpoint = builder.build()?;
        let IssueWrapper { issue } = self
            .inner
            .clone()
            .json_response_body::<_, IssueWrapper<Issue>>(&endpoint)
            .await?;
        serde_json::to_string_pretty(&issue).map_err(CoreError::from)
    }

    pub async fn update_issue(
        &self,
        id: u64,
        subject: Option<&str>,
        description: Option<&str>,
        status_id: Option<u64>,
        priority_id: Option<u64>,
        assigned_to_id: Option<u64>,
        estimated_hours: Option<f64>,
        notes: Option<&str>,
    ) -> Result<(), CoreError> {
        let mut base = UpdateIssue::builder();
        let builder = base.id(id);
        if let Some(subj) = subject {
            builder.subject(subj.to_owned());
        }
        if let Some(desc) = description {
            builder.description(desc.to_owned());
        }
        if let Some(sid) = status_id {
            builder.status_id(sid);
        }
        if let Some(pid) = priority_id {
            builder.priority_id(pid);
        }
        if let Some(aid) = assigned_to_id {
            builder.assigned_to_id(aid);
        }
        if let Some(eh) = estimated_hours {
            builder.estimated_hours(eh);
        }
        if let Some(n) = notes {
            builder.notes(n.to_owned().into());
        }

        let endpoint = builder.build()?;
        self.inner
            .clone()
            .ignore_response_body(&endpoint)
            .await?;
        Ok(())
    }

    pub async fn delete_issue(&self, id: u64) -> Result<(), CoreError> {
        let endpoint = DeleteIssue::builder().id(id).build()?;
        self.inner
            .clone()
            .ignore_response_body(&endpoint)
            .await?;
        Ok(())
    }

    pub async fn list_projects(
        &self,
        limit: u64,
        offset: u64,
    ) -> Result<String, CoreError> {
        let endpoint = ListProjects::builder().build()?;
        let ResponsePage {
            values: projects,
            total_count,
            ..
        } = self
            .inner
            .clone()
            .json_response_body_page::<_, Project>(&endpoint, offset, limit)
            .await?;

        serde_json::to_string_pretty(&serde_json::json!({
            "total_count": total_count,
            "projects": projects,
        }))
        .map_err(CoreError::from)
    }

    pub async fn get_project(&self, id: u64) -> Result<String, CoreError> {
        let endpoint = GetProject::builder()
            .project_id_or_name(id.to_string())
            .build()?;
        let ProjectWrapper { project } = self
            .inner
            .clone()
            .json_response_body::<_, ProjectWrapper<Project>>(&endpoint)
            .await?;
        serde_json::to_string_pretty(&project).map_err(CoreError::from)
    }

    pub async fn list_users(
        &self,
        limit: u64,
        offset: u64,
    ) -> Result<String, CoreError> {
        let endpoint = ListUsers::builder().build()?;
        let ResponsePage {
            values: users,
            total_count,
            ..
        } = self
            .inner
            .clone()
            .json_response_body_page::<_, User>(&endpoint, offset, limit)
            .await?;

        serde_json::to_string_pretty(&serde_json::json!({
            "total_count": total_count,
            "users": users,
        }))
        .map_err(CoreError::from)
    }

    pub async fn get_user(&self, id: u64) -> Result<String, CoreError> {
        let endpoint = GetUser::builder().id(id).build()?;
        let UserWrapper { user } = self
            .inner
            .clone()
            .json_response_body::<_, UserWrapper<User>>(&endpoint)
            .await?;
        serde_json::to_string_pretty(&user).map_err(CoreError::from)
    }

    pub async fn list_time_entries(
        &self,
        project_id: Option<String>,
        spent_on: Option<String>,
        limit: u64,
        offset: u64,
    ) -> Result<String, CoreError> {
        let mut builder = ListTimeEntries::builder();
        if let Some(pid) = project_id {
            builder.project_id_or_name(pid);
        }
        if let Some(so) = spent_on {
            let date = parse_date(&so)?;
            builder.spent_on(redmine_api::api::DateFilter::ExactMatch(date));
        }

        let endpoint = builder.build()?;
        let ResponsePage {
            values: entries,
            total_count,
            ..
        } = self
            .inner
            .clone()
            .json_response_body_page::<_, TimeEntry>(&endpoint, offset, limit)
            .await?;

        serde_json::to_string_pretty(&serde_json::json!({
            "total_count": total_count,
            "time_entries": entries,
        }))
        .map_err(CoreError::from)
    }

    pub async fn create_time_entry(
        &self,
        issue_id: Option<u64>,
        project_id: Option<u64>,
        hours: f64,
        activity_id: Option<u64>,
        comments: Option<&str>,
        spent_on: Option<&str>,
    ) -> Result<String, CoreError> {
        let mut base = CreateTimeEntry::builder();
        let builder = base.hours(hours);
        if let Some(iid) = issue_id {
            builder.issue_id(iid);
        }
        if let Some(pid) = project_id {
            builder.project_id(pid);
        }
        if let Some(aid) = activity_id {
            builder.activity_id(aid);
        }
        if let Some(c) = comments {
            builder.comments(c.to_owned().into());
        }
        if let Some(so) = spent_on {
            let date = time::Date::parse(&so, &time::format_description::well_known::Iso8601::DEFAULT)
                .map_err(|e| CoreError::TimeParse(so.to_owned(), e))?;
            builder.spent_on(date);
        }

        let endpoint = builder.build()?;
        let TimeEntryWrapper { time_entry } = self
            .inner
            .clone()
            .json_response_body::<_, TimeEntryWrapper<TimeEntry>>(&endpoint)
            .await?;
        serde_json::to_string_pretty(&time_entry).map_err(CoreError::from)
    }
}

fn parse_issue_status_filter(s: &str) -> redmine_api::api::issues::IssueStatusFilter {
    match s.to_lowercase().as_str() {
        "open" => redmine_api::api::issues::IssueStatusFilter::Open,
        "closed" => redmine_api::api::issues::IssueStatusFilter::Closed,
        "all" => redmine_api::api::issues::IssueStatusFilter::All,
        _ => {
            if let Ok(id) = s.parse::<u64>() {
                redmine_api::api::issues::IssueStatusFilter::TheseStatuses(vec![id])
            } else {
                redmine_api::api::issues::IssueStatusFilter::Open
            }
        }
    }
}

fn parse_date(s: &str) -> Result<time::Date, CoreError> {
    time::Date::parse(s, &time::macros::format_description!("[year]-[month]-[day]"))
        .map_err(|e| CoreError::TimeParse(s.to_owned(), e))
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JiraHookInfo {
    #[serde(rename = "webhookEvent")]
    pub(crate) web_hook_event: String,

    #[serde(rename = "issue_event_type_name")]
    pub(crate) issue_event_type_name: Option<String>,

    #[serde(rename = "issue")]
    pub(crate) issue: Issue,

    #[serde(rename = "changelog")]
    pub(crate) chang_log: Option<ChangeLog>,

    #[serde(rename = "comment")]
    pub(crate) comment: Option<CommentElement>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeLog {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub(crate) items: Option<Vec<Item>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CommentElement {
    #[serde(rename = "body")]
    pub(crate) body: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    #[serde(rename = "field")]
    pub(crate) field: String,

    #[serde(rename = "fromString")]
    pub(crate) from_string: Option<String>,

    #[serde(rename = "toString")]
    pub(crate) to_string: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Issue {
    #[serde(rename = "key")]
    pub(crate) key: String,
    #[serde(rename = "fields")]
    pub(crate) fields: IssueFields,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueFields {
    #[serde(rename = "issuetype")]
    pub(crate) issue_type: IssueType,

    #[serde(rename = "fixVersions")]
    pub(crate) fix_versions: Option<Vec<FixVersion>>,
    #[serde(rename = "priority")]
    pub(crate) priority: Priority,

    #[serde(rename = "customfield_10100")]
    pub(crate) checker: Option<User>,

    #[serde(rename = "assignee")]
    pub(crate) assignee: User,

    #[serde(rename = "status")]
    pub(crate) status: Status,

    #[serde(rename = "components")]
    pub(crate) components: Option<Vec<Component>>,

    #[serde(rename = "customfield_10204")]
    pub(crate) sprint: Option<Vec<String>>,

    #[serde(rename = "summary")]
    pub(crate) summary: Option<String>,

    #[serde(rename = "reporter")]
    pub(crate) reporter: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "displayName")]
    pub(crate) display_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Priority {
    #[serde(rename = "name")]
    pub(crate) name: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Component {
    #[serde(rename = "name")]
    pub(crate) name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FixVersion {
    #[serde(rename = "name")]
    pub(crate) name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueType {
    #[serde(rename = "name")]
    pub(crate) name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    #[serde(rename = "name")]
    pub(crate) name: String,
}

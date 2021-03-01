use super::structure::JiraHookInfo;

pub trait JiraInterface {
    fn hook_event(&self) -> String;
    fn event_type(&self) -> Option<String>;

    fn issue_type(&self) -> String;
    fn issue_id(&self) -> String;
    fn status(&self) -> String;
    fn priority(&self) -> String;

    fn assignee(&self) -> (String, String);
    fn reporter(&self) -> (String, String);
    fn checker(&self) -> Option<(String, String)>;

    fn summary(&self) -> Option<String>;
    fn comment(&self) -> Option<String>;
    fn model(&self) -> Option<String>;

    fn sprint(&self) -> Option<String>;
    fn fix_versions(&self) -> Option<String>;
}

impl JiraInterface for JiraHookInfo {
    fn hook_event(&self) -> String {
        self.web_hook_event.clone()
    }
    fn event_type(&self) -> Option<String> {
        self.issue_event_type_name.clone()
    }
    fn issue_type(&self) -> String {
        self.issue.fields.issue_type.name.clone()
    }
    fn issue_id(&self) -> String {
        self.issue.key.clone()
    }
    fn status(&self) -> String {
        self.issue.fields.status.name.clone()
    }
    fn priority(&self) -> String {
        self.issue.fields.priority.name.clone()
    }
    fn assignee(&self) -> (String, String) {
        (
            self.issue.fields.assignee.display_name.clone(),
            self.issue.fields.assignee.display_name.clone(),
        )
    }
    fn reporter(&self) -> (String, String) {
        (
            self.issue.fields.reporter.display_name.clone(),
            self.issue.fields.reporter.display_name.clone(),
        )
    }
    fn checker(&self) -> Option<(String, String)> {
        match &self.issue.fields.checker {
            Some(user) => return Some((user.display_name.clone(), user.display_name.clone())),
            None => return None,
        }
    }
    fn summary(&self) -> Option<String> {
        self.issue.fields.summary.clone()
    }
    fn comment(&self) -> Option<String> {
        match &self.comment {
            Some(comment) => return Some(comment.body.clone()),
            None => return None,
        }
    }
    fn model(&self) -> Option<String> {
        match &self.issue.fields.components {
            Some(components) => {
                return Some(
                    components
                        .into_iter()
                        .fold("".to_string(), |acc, item| acc + &item.name),
                )
            }
            None => return None,
        }
    }
    fn sprint(&self) -> Option<String> {
        match &self.issue.fields.sprint {
            None => return None,
            Some(sprint) => {
                return Some(
                    sprint
                        .into_iter()
                        .fold("".to_string(), |acc, item| acc + &item),
                )
            }
        }
    }
    fn fix_versions(&self) -> Option<String> {
        match &self.issue.fields.fix_versions {
            None => return None,
            Some(fix_versions) => {
                return Some(
                    fix_versions
                        .into_iter()
                        .fold("".to_string(), |acc, item| acc + &item.name),
                )
            }
        }
    }
}

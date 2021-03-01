extern crate regex;
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
            self.issue.fields.assignee.name.clone(),
            self.issue.fields.assignee.display_name.clone(),
        )
    }
    fn reporter(&self) -> (String, String) {
        (
            self.issue.fields.reporter.name.clone(),
            self.issue.fields.reporter.display_name.clone(),
        )
    }
    fn checker(&self) -> Option<(String, String)> {
        match &self.issue.fields.checker {
            None => return None,
            Some(user) => return Some((user.name.clone(), user.display_name.clone())),
        }
    }
    fn summary(&self) -> Option<String> {
        self.issue.fields.summary.clone()
    }
    fn comment(&self) -> Option<String> {
        match &self.comment {
            None => return None,
            Some(comment) => return Some(comment.body.clone()),
        }
    }
    fn model(&self) -> Option<String> {
        match &self.issue.fields.components {
            None => return None,
            Some(components) => {
                return Some(
                    components
                        .into_iter()
                        .fold("".to_string(), |acc, item| acc + &item.name),
                )
            }
        }
    }
    fn sprint(&self) -> Option<String> {
        match &self.issue.fields.sprint {
            None => return None,
            Some(sprint) => {
                let reg = regex::Regex::new(r"(Sprint+ \d+\.\d+\.\d+)").unwrap();

                return Some(
                    sprint
                        .into_iter()
                        .map(|v| {
                            if let Some(res) = reg.find(v) {
                                return res.as_str();
                            } else {
                                return "";
                            }
                        })
                        .fold("".to_string(), |acc, item| acc + item),
                );
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

#[test]
fn test_re() {
    let reg = regex::Regex::new(r"(Sprint+ \d+\.\d+\.\d+)").unwrap();
    let cap=reg.find("com.atlassian.greenhopper.service.sprint.Sprint@9254288[id=54,rapidViewId=16,state=FUTURE,name=Sprint 2021.03.12,startDate=<null>,endDate=<null>,completeDate=<null>,sequence=54,goal=<null>]").unwrap();
    println!("reg: {:#?}", cap.as_str());
}

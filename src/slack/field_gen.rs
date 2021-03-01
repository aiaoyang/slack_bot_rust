use std::vec;

use crate::context::Context;
use crate::formatter::ToString;
use crate::jira::traits::JiraInterface;

use super::block_struct::Block;

pub fn gen_all_block<T, J>(c: &T, j: &J) -> Vec<Block>
where
    T: Context,
    J: JiraInterface,
{
    vec![
        Block::new_text(vec![action_field_string(c, j)].to_string()),
        Block::new_text(vec![users_field_string(c, j)].to_string()),
        Block::new_text(vec![issue_field_string(c, j)].to_string()),
        Block::new_divider(),
    ]
}

fn action_field_string<T: Context, J: JiraInterface>(_c: &T, j: &J) -> String {
    vec![_c.to_string(), j.hook_event(), j.issue_id()].to_string()
}

fn users_field_string<T: Context, J: JiraInterface>(_c: &T, j: &J) -> String {
    let (
        (_assignee_name, assignee_display_name),
        (_reporter_name, reporter_display_name),
        (_checker_name, checker_display_name),
    ) = (
        j.assignee(),
        j.reporter(),
        j.checker().unwrap_or(("".to_string(), "".to_string())),
    );

    let v = vec![
        assignee_display_name,
        reporter_display_name,
        checker_display_name,
    ];

    v.to_string()
}

fn issue_field_string<T: Context, J: JiraInterface>(_c: &T, j: &J) -> String {
    let mut v = Vec::new();

    v.push(j.status());
    v.push(j.priority());
    v.push(j.issue_type());

    if let Some(model) = j.model() {
        v.push(model);
    };

    if let Some(fix_version) = j.fix_versions() {
        v.push(fix_version);
    };

    if let Some(sprint) = j.sprint() {
        v.push(sprint);
    };

    v.to_string()
}

use std::vec;

use crate::context::Context;
use crate::formatter::ToString;
use crate::formatter::*;
use crate::jira::traits::JiraInterface;
use crate::slack::structure::{AppMsg, Block};

pub fn gen_msg<T, J>(c: &T, j: &J) -> AppMsg
where
    T: Context,
    J: JiraInterface,
{
    AppMsg::new(gen_all_block(c, j))
}

pub fn gen_all_block<T, J>(c: &T, j: &J) -> Vec<Block>
where
    T: Context,
    J: JiraInterface,
{
    vec![
        Block::new_text(vec![action_field_string(c, j)].to_string()),
        Block::new_text(vec![issue_field_string(c, j)].to_string()),
        Block::new_text(vec![users_field_string(c, j)].to_string()),
        Block::new_divider(),
    ]
}

fn action_field_string<T: Context, J: JiraInterface>(_c: &T, j: &J) -> String {
    let link_str = j.issue_id().link(
        format!(
            "{}-{}",
            j.issue_id().as_str(),
            j.summary().unwrap_or("".to_string()).as_str()
        )
        .as_str(),
    );
    vec![
        _c.to_string(),
        "-".into(),
        j.hook_event(),
        "-".into(),
        j.issue_id(),
        "-".into(),
        link_str,
    ]
    .to_string()
}

fn issue_field_string<T: Context, J: JiraInterface>(_c: &T, j: &J) -> String {
    let v = vec![
        //
        "\n>状态:".to_string(),
        j.status().code(),
        //
        "\t优先级:".to_string(),
        j.priority().bold(),
        //
        "\n>类型:".to_string(),
        j.issue_type().bold(),
        //
        "\t模块:".to_string(),
        j.model().unwrap_or("无".to_string()),
        //
        "\n>修复的版本:".to_string(),
        j.fix_versions().unwrap_or("无".to_string()),
        //
        "\tsprint:".to_string(),
        j.sprint().unwrap_or("无".to_string()),
    ];

    v.to_string()
}

fn users_field_string<T: Context, J: JiraInterface>(_c: &T, j: &J) -> String {
    let (
        (_assignee_name, assignee_display_name),
        (_reporter_name, reporter_display_name),
        (_checker_name, checker_display_name),
    ) = (
        j.assignee(),
        j.reporter(),
        j.checker().unwrap_or(("无".to_string(), "无".to_string())),
    );

    let v = vec![
        "经办人:".to_string(),
        assignee_display_name,
        "\t报告人:".to_string(),
        reporter_display_name,
        "\t验收人:".to_string(),
        checker_display_name,
    ];

    v.to_string()
}

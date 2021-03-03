use std::{collections::HashMap, vec};

use crate::context::Context;
use crate::formatter::ToString;
use crate::formatter::*;
use crate::jira::traits::JiraInterface;
use crate::slack::structure::{AppMsg, Block};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref ACTION_HASHMAP: HashMap<String, String> = {
        let mut hm: HashMap<String, String> = HashMap::new();
        hm.insert("issue_created".to_string(), "创建问题".to_string());
        hm.insert("issue_resolved".to_string(), "解决问题".to_string());
        hm.insert("issue_commented".to_string(), "添加评论".to_string());
        hm.insert("issue_closed".to_string(), "关闭问题".to_string());
        hm.insert("issue_reopened".to_string(), "重开问题".to_string());
        hm.insert("issue_assigned".to_string(), "修改经办人".to_string());
        hm
    };
}

pub fn gen_msg<T, J>(c: &T, j: &J) -> AppMsg
where
    T: Context,
    J: JiraInterface,
{
    AppMsg::new(gen_all_block(c, j).unwrap())
}

pub fn gen_all_block<T, J>(c: &T, j: &J) -> Option<Vec<Block>>
where
    T: Context,
    J: JiraInterface,
{
    if let Some(ref event_type) = j.event_type() {
        if let Some(_) = ACTION_HASHMAP.get(event_type) {
            let mut blocks = vec![
                Block::new_text(vec![action_field_string(c, j)].to_string()),
                Block::new_text(vec![issue_field_string(c, j)].to_string()),
                Block::new_divider(),
            ];
            if let Some(c) = j.comment() {
                blocks.insert(1, Block::new_text(vec![c].to_string()));
            }
            return Some(blocks);
        }
    }

    None
}

fn action_field_string<T: Context, J: JiraInterface>(_c: &T, j: &J) -> String {
    let mut action = if let Some(ref event_type) = j.event_type() {
        ACTION_HASHMAP.get(event_type).unwrap().to_owned()
    } else {
        "未定义行为".to_string()
    };
    let link_str = j.issue_id().link(
        format!(
            "{}-{}",
            j.issue_id(),
            j.summary().unwrap_or("".to_string()).as_str()
        )
        .as_str(),
    );

    vec![_c.to_string().tab(), action.tab(), link_str].to_string()
}

fn issue_field_string<T: Context, J: JiraInterface>(_c: &T, j: &J) -> String {
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
        //
        "状态:".to_string().line(),
        j.status().code(),
        //
        "\t优先级:".to_string(),
        j.priority().bold(),
        //
        "类型:".to_string().line(),
        j.issue_type().bold(),
        //
        "\t模块:".to_string(),
        j.model().unwrap_or("无".to_string()),
        //
        "修复的版本:".to_string().line(),
        j.fix_versions().unwrap_or("无".to_string()),
        //
        "\tsprint:".to_string(),
        //
        j.sprint().unwrap_or("无".to_string()),
        //
        "经办人:".to_string().line(),
        //
        assignee_display_name,
        //
        "\t报告人:".to_string(),
        //
        reporter_display_name,
        //
        "\t验收人:".to_string(),
        //
        checker_display_name,
    ];

    v.to_string()
}

// fn users_field_string<T: Context, J: JiraInterface>(_c: &T, j: &J) -> String {
//     let (
//         (_assignee_name, assignee_display_name),
//         (_reporter_name, reporter_display_name),
//         (_checker_name, checker_display_name),
//     ) = (
//         j.assignee(),
//         j.reporter(),
//         j.checker().unwrap_or(("无".to_string(), "无".to_string())),
//     );

//     let v = vec![
//         "经办人:".to_string(),
//         assignee_display_name,
//         "\t报告人:".to_string(),
//         reporter_display_name,
//         "\t验收人:".to_string(),
//         checker_display_name,
//     ];

//     v.to_string().line()
// }

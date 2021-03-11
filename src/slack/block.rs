use std::vec;

use crate::{
    context::Context,
    formatter::*,
    jira::traits::JiraInterface,
    slack::{structure::Block, ACTION_HASHMAP},
};

// pub(crate) fn gen_msg<T, J>(c: &T, j: &J) -> Option<AppMsg>
// where
//     T: Context,
//     J: JiraInterface,
// {
//     if let Some(app_msg) = gen_all_block(c, j) {
//         return Some(AppMsg::new(app_msg));
//     }
//     None
// }

pub(crate) fn gen_all_block<T, J>(c: &T, j: &J) -> Option<Vec<Block>>
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
            if let Some(comment) = j.comment() {
                blocks.insert(1, Block::new_text(vec![comment].to_string()));
            }
            return Some(blocks);
        }
    }

    None
}

pub(self) fn action_field_string<T: Context, J: JiraInterface>(_c: &T, j: &J) -> String {
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
        "状态: ".to_string().line(),
        j.status().code(),
        //
        "\t优先级: ".to_string(),
        j.priority().bold(),
        //
        "类型: ".to_string().line(),
        j.issue_type().bold(),
        //
        "\t模块: ".to_string(),
        j.model().unwrap_or("无".to_string()),
        //
        "修复的版本: ".to_string().line(),
        j.fix_versions().unwrap_or("无".to_string()),
        //
        "\tsprint: ".to_string(),
        //
        j.sprint().unwrap_or("无".to_string()),
        //
        "经办人: ".to_string().line(),
        //
        assignee_display_name,
        //
        "\t报告人: ".to_string(),
        //
        reporter_display_name,
        //
        "\t验收人: ".to_string(),
        //
        checker_display_name,
    ];

    v.to_string()
}

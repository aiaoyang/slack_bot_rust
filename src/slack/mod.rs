pub mod block;
pub mod channel;
pub mod structure;

use lazy_static::lazy_static;
use std::collections::HashMap;

pub const ISSUE_CREATE: &'static str = "issue_created";
pub const ISSUE_RESOLVED: &'static str = "issue_resolved";
pub const ISSUE_COMMENTED: &'static str = "issue_commented";
pub const ISSUE_CLOSED: &'static str = "issue_closed";
pub const ISSUE_REOPENED: &'static str = "issue_reopened";
pub const ISSUE_ASSIGNED: &'static str = "issue_assigned";

lazy_static! {
    pub static ref ACTION_HASHMAP: HashMap<String, String> = {
        let mut hm: HashMap<String, String> = HashMap::new();
        hm.insert(ISSUE_CREATE.to_string(), "创建问题".to_string());
        hm.insert(ISSUE_RESOLVED.to_string(), "解决问题".to_string());
        hm.insert(ISSUE_COMMENTED.to_string(), "添加评论".to_string());
        hm.insert(ISSUE_CLOSED.to_string(), "关闭问题".to_string());
        hm.insert(ISSUE_REOPENED.to_string(), "重开问题".to_string());
        hm.insert(ISSUE_ASSIGNED.to_string(), "修改经办人".to_string());

        hm
    };
}

use crate::{context::Context, jira::traits::JiraInterface, slack::field_gen::gen_all_block};

use self::block_struct::AppMsg;

pub mod block_struct;
pub mod msg_struct;

mod field_gen;

pub fn gen_msg<T, J>(c: &T, j: &J) -> AppMsg
where
    T: Context,
    J: JiraInterface,
{
    AppMsg::new(gen_all_block(c, j))
}

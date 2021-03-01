use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SlackAppMsg {
    blocks: Option<Vec<Block>>,
}
#[derive(Serialize, Deserialize)]
pub struct Block {
    block_type: String,
    text: Option<Text>,
    fields: Vec<Text>,
}

#[derive(Serialize, Deserialize)]
pub struct Text {
    slack_type: SlackType,
    content: String,
}
#[derive(Serialize, Deserialize)]
pub enum SlackType {
    Mrkdwn(String),
    Divider(String),
}

use crate::context::Context;
use crate::jira::traits::JiraInterface;
use crate::slack::block::gen_all_block;

use actix_web::client;
use serde::{Deserialize, Serialize};

const SECTION: &'static str = "section";
const DIVIDER: &'static str = "divider";
const MARKDOWN: &'static str = "mrkdwn";

impl<'a> Msg<'a> {
    fn new(channel: &'a str, text: &'a str, app_msg: AppMsg) -> Self {
        Msg {
            channel,
            text,
            app_msg,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Msg<'a> {
    #[serde(rename = "channel")]
    channel: &'a str,

    #[serde(rename = "text")]
    text: &'a str,

    #[serde(flatten)]
    app_msg: AppMsg,
}

impl AppMsg {
    pub fn new(blocks: Vec<Block>) -> Self {
        AppMsg { blocks }
    }

    pub fn from<T, J>(c: &T, j: &J) -> Option<Self>
    where
        T: Context,
        J: JiraInterface,
    {
        match gen_all_block(c, j) {
            Some(blocks) => return Some(AppMsg { blocks }),
            None => return None,
        }
    }

    pub async fn send(
        &self,
        token: &str,
        post_url: &str,
        user_channel_name: &str,
    ) -> Result<actix_web::HttpResponse, actix_web::Error> {
        use actix_web::http::header::*;

        let c = client::Client::new();

        if let Some(title) = self.blocks.get(0) {
            let title: String = title.clone().into();
            if c.post(post_url)
                .set_header(CONTENT_TYPE, "application/json;charset=utf-8")
                .set_header(AUTHORIZATION, format!("Bearer {}", token))
                .send_json(&Msg::new(user_channel_name, &title, self.clone()))
                .await?
                .status()
                .is_success()
            {
                return Ok(actix_web::web::HttpResponse::Ok().body("ok"));
            }
        }
        Err(actix_web::error::ErrorBadGateway(""))
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AppMsg {
    #[serde(rename = "blocks")]
    blocks: Vec<Block>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Block {
    #[serde(rename = "type")]
    self_type: String,

    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    text: Option<Text>,

    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<Text>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Text {
    #[serde(rename = "type")]
    slack_type: String,

    #[serde(rename = "text")]
    text: String,
}
#[derive(Deserialize, Serialize, Debug)]
struct Divider {
    #[serde(rename = "type")]
    self_type: String,
}

impl Block {
    pub fn new_text<T>(content: T) -> Self
    where
        T: Into<String>,
        T: Clone,
    {
        let c: String = content.into();

        Block {
            self_type: SECTION.into(),
            text: Some(Text {
                slack_type: MARKDOWN.into(),
                text: c,
            }),
            fields: None,
        }
    }

    pub fn new_field<T>(fields: Vec<T>) -> Self
    where
        T: Into<String>,
        T: Clone,
    {
        let mut fields_tmp: Vec<Text> = Vec::new();

        for text in fields
            .iter()
            .map(|text| {
                let string_text: String = text.clone().into();
                string_text
            })
            .into_iter()
        {
            &fields_tmp.push(Text {
                slack_type: MARKDOWN.into(),
                text,
            });
        }

        Block {
            self_type: SECTION.into(),
            text: None,
            fields: Some(fields_tmp),
        }
    }

    pub fn new_divider() -> Self {
        Block {
            self_type: DIVIDER.into(),
            text: None,
            fields: None,
        }
    }
}

impl From<Block> for String {
    fn from(b: Block) -> Self {
        let t_struct = b.text.unwrap_or(Text {
            slack_type: "".to_string(),
            text: "".to_string(),
        });
        t_struct.text
    }
}

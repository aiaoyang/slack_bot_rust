use reqwest::blocking::{Client, Response};
use reqwest::{header::HeaderMap, Error};
use serde::{Deserialize, Serialize};

const SECTION: &'static str = "section";
const DIVIDER: &'static str = "divider";
const MARKDOWN: &'static str = "mrkdwn";

trait Context {
    fn to_string() -> String;
    fn todo();
}
impl<'a> Msg<'a> {
    fn new(channel: &'a str, text: &'a str, app_msg: AppMsg) -> Self {
        Msg {
            channel,
            text,
            app_msg,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
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
    pub fn send(self) -> Result<Response, Error> {
        let c = Client::new();
        let mut header_map = HeaderMap::new();

        header_map.insert(
            "Content-Type",
            "application/json;charset=utf-8".parse().unwrap(),
        );

        header_map.insert(
            "Authorization",
            "Bearer xoxb-1626838453092-1657930941057-Nlx7xJiF4CRer7hjgLURCaqz"
                .parse()
                .unwrap(),
        );

        c.post("https://slack.com/api/chat.postMessage")
            .headers(header_map)
            .json(&Msg::new("U01JERHHPEY", "text", self))
            .send()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppMsg {
    #[serde(rename = "blocks")]
    blocks: Vec<Block>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Block {
    #[serde(rename = "type")]
    self_type: String,

    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    text: Option<Text>,

    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<Text>>,
}

#[derive(Deserialize, Serialize, Debug)]
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

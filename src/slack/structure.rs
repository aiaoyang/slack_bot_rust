use serde::{Deserialize, Serialize};

const SECTION: &'static str = "section";
const DIVIDER: &'static str = "divider";
const MARKDOWN: &'static str = "mrkdwn";

trait Context {
    fn to_string() -> String;
    fn todo();
}
#[derive(Deserialize, Serialize, Debug)]
pub struct AppMsg {
    #[serde(rename = "blocks")]
    blocks: Vec<Block>,
}

impl AppMsg {
    pub fn new(blocks: Vec<Block>) -> Self {
        AppMsg { blocks }
    }
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

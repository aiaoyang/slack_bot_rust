use std::collections::HashMap;

use reqwest::{blocking::*, header::HeaderMap};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SlackUserList {
    #[serde(rename = "ok")]
    ok: bool,

    #[serde(rename = "members")]
    members: Vec<SlackUser>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SlackUser {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "id")]
    id: String,
}

impl SlackUserList {
    pub fn new() -> Result<HashMap<String, String>, reqwest::Error> {
        get_user_channel()
    }
}

fn get_user_channel() -> Result<HashMap<String, String>, reqwest::Error> {
    let c = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        "application/x-www-form-urlencoded".parse().unwrap(),
    );

    headers.insert(
        "Authorization",
        "Bearer xoxb-1626838453092-1657930941057-r4g8fIz2k6GArfq3tc2l0Y5g"
            .parse()
            .unwrap(),
    );

    match c
        .get("https://slack.com/api/users.list")
        .headers(headers)
        .send()
    {
        Ok(resp) => match resp.json::<SlackUserList>() {
            Ok(user_list) => {
                let mut hm: HashMap<String, String> = HashMap::new();
                for user in user_list.members.into_iter() {
                    hm.insert(user.name, user.id);
                }
                return Ok(hm);
            }
            Err(e) => return Err(e),
        },
        Err(e) => return Err(e),
    }
}

#[test]
fn test_user_channel() {
    let user_list = get_user_channel().unwrap();
    println!("{:?}", user_list)
}

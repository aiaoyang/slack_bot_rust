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
    pub fn new(token: &str, url: &str) -> Result<HashMap<String, String>, reqwest::Error> {
        get_user_channel(token, url)
    }
}

// 获取 用户:用户ID 键值对
fn get_user_channel(token: &str, url: &str) -> Result<HashMap<String, String>, reqwest::Error> {
    let c = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        "application/x-www-form-urlencoded".parse().unwrap(),
    );

    headers.insert(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );

    match c.get(url).headers(headers).send() {
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
    let user_list = get_user_channel("", "").unwrap();
    println!("{:?}", user_list)
}

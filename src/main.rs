use std::collections::HashMap;

use config::*;
use serde::{Deserialize, Serialize};

use slack_bot::jira::structure::JiraHookInfo;
use slack_bot::slack::channel::SlackUserList;
use slack_bot::slack::structure::AppMsg;
use slack_bot::userdb::get_users;
use slack_bot::{context::MyContext, jira::traits::JiraInterface};
#[macro_use]
extern crate lazy_static;

use actix_web::web::Json;
use actix_web::{post, App, Error, HttpRequest, HttpResponse, HttpServer, Result};

use qstring::QString;

lazy_static! {
    // 全局配置文件
    static ref GLOBAL_CONFIG: MyConfig = MyConfig::new().unwrap();

    // 全局用户名哈希表
    static ref HASHCONFIG: HashMap<String, String> = get_users(&GLOBAL_CONFIG.ldap.url, &GLOBAL_CONFIG.ldap.base_dn);

    // 全局用户ID哈希表
    static ref SLACK_CHANNEL: HashMap<String, String> = SlackUserList::new(
        &GLOBAL_CONFIG.slack.token,
        &GLOBAL_CONFIG.slack.channel_search_url
    ).unwrap();
}
#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
    slack: Slack,
    ldap: Ldap,
}

#[derive(Debug, Serialize, Deserialize)]
struct Ldap {
    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "baseDn")]
    base_dn: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Slack {
    #[serde(rename = "channelSearchUrl")]
    channel_search_url: String,

    #[serde(rename = "msgSendUrl")]
    msg_send_url: String,

    #[serde(rename = "token")]
    token: String,

    #[serde(rename = "adminUsers")]
    admin_users: Option<Vec<String>>,
}

impl MyConfig {
    fn new() -> Result<Self, ConfigError> {
        let mut c = Config::default();
        c.merge(File::with_name("config.yaml"))?;
        c.try_into()
        // todo!()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(jira_hook))
        .bind("0.0.0.0:9998")?
        .run()
        .await
}

#[post("/jirahook")]
async fn jira_hook(req: HttpRequest, jira_info: Json<JiraHookInfo>) -> Result<HttpResponse, Error> {
    println!("here i im");
    let query_str = req.query_string();
    let qs = QString::from(query_str);

    let action_user = qs.get("user_id").unwrap_or("未定义行为");
    let default_action_user = "JIRA机器人".to_string();
    let action_user = HASHCONFIG.get(action_user).unwrap_or(&default_action_user);

    let admin_users = GLOBAL_CONFIG
        .slack
        .admin_users
        .as_ref()
        .unwrap()
        .iter()
        .map(|user_name| SLACK_CHANNEL.get(user_name).unwrap())
        .collect::<Vec<&String>>();

    let to_user = SLACK_CHANNEL.get(&jira_info.assignee().0);
    println!("action_user: {}, send_to: {:#?}", &action_user, &to_user);

    let ctx = MyContext::from(action_user.to_string());

    match AppMsg::from(&ctx, &jira_info.0) {
        Some(app_msg) => {
            if let Ok(_) = serde_json::to_string(&app_msg) {
                for user_id in admin_users {
                    if let Ok(_) = app_msg
                        .send(
                            &GLOBAL_CONFIG.slack.token,
                            &GLOBAL_CONFIG.slack.msg_send_url,
                            user_id,
                        )
                        .await
                    {};
                }
                match to_user {
                    Some(user) => {
                        if let Ok(_) = app_msg
                            .send(
                                &GLOBAL_CONFIG.slack.token,
                                &GLOBAL_CONFIG.slack.msg_send_url,
                                &user,
                            )
                            .await
                        {}
                    }
                    None => (),
                }
            } else {
                println!("{}", "error");
            }
        }
        None => println!("nothing happen"),
    }

    Ok(HttpResponse::Ok().body("response"))
}

#[cfg(test)]
mod test {
    #[test]
    fn test_config_read() {
        let conf = crate::MyConfig::new();
        println!("{:#?}", conf);
    }
}

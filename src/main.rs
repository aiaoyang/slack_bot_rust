use std::collections::HashMap;

use config::*;
use serde::{Deserialize, Serialize};

use slack_bot::{
    context::MyContext,
    jira::{structure::JiraHookInfo, traits::JiraInterface},
    slack::{channel::SlackUserList, structure::AppMsg},
    userdb::get_users,
};

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
    let query_str = req.query_string();
    let qs = QString::from(query_str);

    let action_user = qs.get("user_id").unwrap_or("未定义行为");
    let default_action_user = "JIRA机器人".to_string();

    // jirahook触发动作的用户
    let action_user = HASHCONFIG.get(action_user).unwrap_or(&default_action_user);

    // 问题创建与重新打开分配用户
    let admin_users = GLOBAL_CONFIG.slack.admin_users.as_ref().unwrap();

    let ctx = MyContext::from(action_user.to_string());

    // 所有权变量
    let j = jira_info.0;

    let ((assignee_name, _), (reporter_name, _), (checker_name, _)) = (
        j.assignee(),
        j.reporter(),
        j.checker().unwrap_or(("无".to_string(), "无".to_string())),
    );

    let _yangjd = "yangjiangdong".to_string();
    // 初始化要发送的用户列表，经办人和报告人必选
    let mut send_user: HashMap<&String, bool> = HashMap::new();
    send_user.insert(&assignee_name, true);
    send_user.insert(&_yangjd, true);
    send_user.insert(&reporter_name, true);

    println!("action: {}", &j.event_type().as_ref().unwrap().as_str());

    match &j.event_type() {
        Some(event_type) => match event_type.as_str() {
            "issue_reopened" | "issue_created" => {
                let _ = admin_users
                    .iter()
                    .map(|user_name| {
                        send_user.insert(user_name, true);
                    })
                    .collect::<()>();
            }

            "issue_resolved" => {
                send_user.insert(&checker_name, true);
            }
            "issue_updated" | "issue_generic" => {
                return Ok(HttpResponse::Ok().body("response"));
            }

            _ => {}
        },
        None => (),
    }

    let finaly_send_user = send_user
        .iter()
        .filter_map(|(user_name, _)| SLACK_CHANNEL.get(user_name.clone()))
        .collect::<Vec<&String>>();

    println!("send user list: {:?}", &finaly_send_user);

    if let Some(app_msg) = AppMsg::from(&ctx, &j) {
        if let Ok(_) = serde_json::to_string(&app_msg) {
            for user_id in finaly_send_user {
                match app_msg
                    .send(
                        &GLOBAL_CONFIG.slack.token,
                        &GLOBAL_CONFIG.slack.msg_send_url,
                        user_id,
                    )
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        println!("send error: {:?}", e)
                    }
                };
            }
            println!("send done");
        } else {
            println!("json to string error");
        }
        // None => println!("nothing happen"),
    } else {
        println!("new appmsg error")
    };

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

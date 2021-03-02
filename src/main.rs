use std::collections::HashMap;

use slack_bot::context::MyContext;
use slack_bot::jira::structure::JiraHookInfo;
use slack_bot::slack::structure::AppMsg;
use slack_bot::userdb::get_users;

#[macro_use]
extern crate lazy_static;
extern crate qstring;
use qstring::QString;

use actix_web::web::Json;
use actix_web::{post, App, Error, HttpRequest, HttpResponse, HttpServer, Result};

lazy_static! {
    static ref HASHCONFIG: HashMap<String, String> = get_users();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(jira_hook))
        .bind("0.0.0.0:9998")?
        .run()
        .await
}

#[post("/jirahook")]
async fn jira_hook(req: HttpRequest, content: Json<JiraHookInfo>) -> Result<HttpResponse, Error> {
    let query_str = req.query_string();
    let qs = QString::from(query_str);
    let action_user = qs.get("user_id").unwrap_or("未定义行为");
    let default_action_user = "svnpush".to_string();
    let action_user = HASHCONFIG.get(action_user).unwrap_or(&default_action_user);
    println!("{}", &action_user);

    let ct = MyContext {
        s: action_user.to_string(),
    };

    let app_msg = AppMsg::from(&ct, &content.0);
    // let default = String::from("yangjiangdong");
    // let channel = HASHCONFIG.get(&ct.to_string()).unwrap_or(&default).as_str();

    if let Ok(encode_json_str) = serde_json::to_string(&app_msg) {
        println!("{}", encode_json_str);
        if let Ok(result) = app_msg.send("U01JERHHPEY") {
            println!("{:#?}", result.text());
        }
    } else {
        println!("{}", "error");
    }

    Ok(HttpResponse::Ok().body("response"))
}

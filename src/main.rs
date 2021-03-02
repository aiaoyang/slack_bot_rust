#[macro_use]
extern crate lazy_static;
extern crate qstring;
use qstring::QString;

use actix_web::{post, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use std::collections::HashMap;

use slack_bot::context::MyContext;
use slack_bot::jira::structure::JiraHookInfo;
use slack_bot::slack::generator::gen_msg;
use slack_bot::userdb::get_users;

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
async fn jira_hook(req: HttpRequest, content: String) -> Result<HttpResponse, Error> {
    let query_str = req.query_string();
    let qs = QString::from(query_str);
    let action_user = qs.get("user_id").unwrap_or("未定义行为");

    println!("action_user:{}", &action_user);

    let json_decode = serde_json::from_str::<JiraHookInfo>(&content);
    match json_decode {
        Ok(json_str) => {
            let ct = MyContext {
                s: action_user.to_string(),
            };

            let app_msg = gen_msg(&ct, &json_str);
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
        }

        Err(error) => {
            println!("Error: {}", error);
        }
    }

    Ok(HttpResponse::Ok().body("response"))
}

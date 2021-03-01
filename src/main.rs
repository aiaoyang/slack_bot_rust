extern crate slack_bot;
use slack_bot::context::MyContext;
use slack_bot::jira::structure::JiraHookInfo;
use slack_bot::slack::generator::gen_msg;

use actix_web::{post, App, Error, HttpResponse, HttpServer, Result};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(jira_hook))
        .bind("0.0.0.0:9998")?
        .run()
        .await
}

#[post("/jirahook")]
async fn jira_hook(info: String) -> Result<HttpResponse, Error> {
    // println!("{}", &info);
    let json_decode = serde_json::from_str::<JiraHookInfo>(&info);
    match json_decode {
        Ok(json_str) => {
            // println!("json string: {:#?}", json_str);

            let ct = MyContext {
                s: "test_context".to_string(),
            };
            let app_msg = gen_msg(&ct, &json_str);
            if let Ok(encode_json_str) = serde_json::to_string(&app_msg) {
                println!("{}", encode_json_str);
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

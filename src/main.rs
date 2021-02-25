extern crate slack_bot;
use actix_web::{post, App, Error, HttpResponse, HttpServer, Result};

// use serde::{Deserialize, Serialize};
use slack_bot::jira::structure::JiraHookInfo;
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
            println!("json string: {:#?}", json_str);
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
    Ok(HttpResponse::Ok().body("response"))
}

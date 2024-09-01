use std::collections::HashMap;
use std::env;

use axum::extract::Query;
use axum::routing::get;
use axum::Router;

async fn func(Query(params): Query<HashMap<String, String>>) -> String {
    println!("got new request!");
    match params.get("name") {
        Some(name) => format!("Hello, {}. This HTTP triggered function executed successfully.", name),
        None => "This HTTP triggered function executed successfully. Pass a name in the query string for a personalized response.".to_owned(),
    }
}

#[tokio::main]
async fn main() {
    println!("Starting app...");

    let port: u16 = match env::var("FUNCTIONS_CUSTOMHANDLER_PORT") {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => {
            println!("Did not find handler port...");
            3000
        }
    };

    let address = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    println!("Listening on {address}");

    let router = Router::new().route("/api/TelegramTrigger", get(func));
    axum::serve(listener, router).await.unwrap();
}

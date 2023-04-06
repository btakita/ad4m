mod graphql;
mod js_core;

use js_core::JsCore;

#[tokio::main]
async fn main() {
    let mut js_core = JsCore::start();
    js_core.initialized().await;

    //let fut_res = tokio::try_join!(graphql::start_server(), js_core::JsCore::run());
    //if let Err(error) = fut_res {
    //    eprintln!("error: {}", error);
    //}
}

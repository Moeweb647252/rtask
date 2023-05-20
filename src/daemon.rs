use std::sync::RwLock;

use crate::types::*;
use crate::utils::*;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer, Responder};
use tokio::runtime::Runtime;

async fn hello() -> impl Responder {
  nsucc(200, "Hello world")
}

async fn get_entries(data: ReqData, state: RS) -> impl Responder {
  let rtodo = state.rtodo.read().unwrap();
  if !check_token(&data, &rtodo) {
    return nerr(100, "Invalid token");
  }
  let entries = serde_json::to_string(&rtodo.get_entries()).unwrap();
  nsucc(200, entries)
}

async fn validate_token(data: ReqData, state: RS) -> impl Responder {
  let rtodo = state.rtodo.read().unwrap();
  if !check_token(&data, &rtodo) {
    return nerr(100, "Invalid token");
  }
  nsucc(200, "Valid token")
}

pub fn start_server(rtodo: Rtodo) -> std::io::Result<()> {
  let rt = Runtime::new().unwrap();
  let addr = rtodo.config.address.clone();
  rt.block_on(async {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let state = web::Data::new(RtodoState {
      rtodo: RwLock::new(rtodo),
    });
    HttpServer::new(move || {
      App::new()
        .wrap(Cors::default().allow_any_origin())
        .wrap(Logger::default())
        .wrap(Logger::new("%a"))
        .app_data(state.clone())
        .service(
          web::scope("/api")
            .route(
              "/",
              web::get().to(|| async { String::from("Hello, rtodo!") }),
            )
            .route("/getEntries", web::post().to(get_entries))
            .route("/validateToken", web::post().to(validate_token)),
        )
        .service(web::resource("/").route(web::get().to(hello)))
    })
    .bind(addr.unwrap_or("0.0.0.0:6472".to_string()))?
    .run()
    .await
  })
}

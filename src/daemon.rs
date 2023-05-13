use serde_json::Serializer;
use std::sync::RwLock;

use crate::types::*;
use crate::utils::*;

use actix_web::{
  web::{self, Json},
  App, HttpServer, Responder,
};
use serde_json::Value;
use tokio::runtime::Runtime;

async fn get_entries(data: ReqData, state: RS) -> impl Responder {
  let rtodo = state.rtodo.read().unwrap();
  if check_token(&data, &rtodo) == false {
    return nerr(100, "Invalid token");
  }
  let entries = serde_json::to_string(&rtodo.get_entries()).unwrap();
  nsucc(200, entries)
}

async fn validate_token(data: ReqData, state: RS) -> impl Responder {
  let rtodo = state.rtodo.read().unwrap();
  if check_token(&data, &rtodo) == false {
    return nerr(100, "Invalid token");
  }
  nsucc(200, "Valid token")
}

pub fn start_server(rtodo: Rtodo) -> std::io::Result<()> {
  let rt = Runtime::new().unwrap();
  let addr = rtodo.config.address.clone();
  rt.block_on(async {
    let state = web::Data::new(RtodoState {
      rtodo: RwLock::new(rtodo),
    });
    HttpServer::new(move || {
      App::new().app_data(state.clone()).service(
        web::scope("/api")
          .route(
            "/",
            web::get().to(|| async { String::from("Hello, rtodo!") }),
          )
          .route("/getEntries", web::post().to(get_entries))
          .route("/validateToken", web::post().to(validate_token)),
      )
    })
    .bind(addr)?
    .run()
    .await
  })
}

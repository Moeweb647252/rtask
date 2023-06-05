use std::sync::{Arc, RwLock};

use crate::types::*;
use crate::utils::*;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer, Responder};
use tokio::runtime::Runtime;

async fn hello() -> impl Responder {
  nsucc(200, "Hello world")
}

async fn get_entries(data: ReqData, state: RS) -> impl Responder {
  let rtodo = get_rtodo_read_gurad(&state).await;
  if !check_token(&data, &rtodo) {
    return nerr(100, "Invalid token");
  }
  nsucc(200, rtodo.get_entries())
}

async fn add_entries(data: ReqDataT<Vec<Entry>>, state: RS) -> impl Responder {
  let mut rtodo = get_rtodo_write_gurad(&state).await;
  if !data.check_token(&rtodo) {
    return nerr(100, "Invalid token");
  }
  for entry in match &data.data {
    Some(d) => d.clone(),
    None => {
      return nerr(100, "Invalid data");
    }
  } {
    rtodo.add_entry(entry);
  }
  nsucc(200, "succeed")
}

async fn delete_entries(data: ReqDataT<Vec<EntryIdentifier>>, state: RS) -> impl Responder {
  let mut rtodo = get_rtodo_write_gurad(&state).await;
  if !data.check_token(&rtodo) {
    return nerr(100, "Invalid token");
  }
  nsucc(200, "succeed")
}

async fn validate_token(data: ReqData, state: RS) -> impl Responder {
  let rtodo = get_rtodo_read_gurad(&state).await;
  if !check_token(&data, &rtodo) {
    return nerr(100, "Invalid token");
  }
  nsucc(200, "Valid token")
}

pub fn start_server(rtodo: Arc<RwLock<Rtodo>>) -> std::io::Result<()> {
  let rt = Runtime::new().unwrap();
  let addr = rtodo.read().unwrap().config.address.clone();
  rt.block_on(async {
    let state = web::Data::new(RtodoState {
      rtodo: rtodo.clone(),
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
            .route("/validateToken", web::post().to(validate_token))
            .route("/addEntry", web::post().to(add_entries)),
        )
        .service(web::resource("/").route(web::get().to(hello)))
    })
    .bind(addr.unwrap_or("0.0.0.0:6472".to_string()))?
    .run()
    .await
  })
}

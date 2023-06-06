use std::sync::{Arc, RwLock};

use crate::types::*;
use crate::utils::*;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer, Responder};
use log::{error, info};
use tokio::runtime::Runtime;

async fn hello() -> impl Responder {
  nsucc(200, "Hello world")
}

async fn validate_token(data: ReqData, state: RS) -> impl Responder {
  let rtodo = get_rtodo_read_gurad(&state).await;
  if !check_token(&data, &rtodo) {
    return nerr(100, "Invalid token");
  }
  nsucc(200, "Valid token")
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
    match rtodo.add_entry(entry) {
      Ok(_) => {}
      Err(e) => {
        return nerr(100, &format!("Failed to add entry: {}", e));
      }
    };
  }
  nsucc(200, "succeed")
}

async fn delete_entries(data: ReqDataT<Vec<EntryIdentifier>>, state: RS) -> impl Responder {
  let mut rtodo = get_rtodo_write_gurad(&state).await;
  if !data.check_token(&rtodo) {
    return nerr(100, "Invalid token");
  }
  for identifier in match &data.data {
    Some(d) => d,
    None => {
      return nerr(100, "Invalid data");
    }
  } {
    match rtodo.delete_entry(identifier) {
      Ok(_) => {}
      Err(e) => {
        return nerr(100, &format!("Failed to delete entry: {}", e));
      }
    };
  }
  nsucc(200, "succeed")
}

async fn get_works(data: ReqData, state: RS) -> impl Responder {
  let rtodo = get_rtodo_read_gurad(&state).await;
  if !check_token(&data, &rtodo) {
    return nerr(100, "Invalid token");
  }
  nsucc(200, &rtodo.works)
}

async fn edit_entry(data: ReqDataT<Entry>, state: RS) -> impl Responder {
  let mut rtodo = get_rtodo_write_gurad(&state).await;
  if !data.check_token(&rtodo) {
    return nerr(100, "Invalid token");
  }
  match &data.data {
    Some(d) => match rtodo.edit_entry(d) {
      Ok(_) => {
        return nsucc(200, "succeed");
      }
      Err(e) => {
        return nerr(100, &format!("Failed to edit entry: {}", e));
      }
    },
    None => {
      return nerr(100, "Invalid data");
    }
  };
}

async fn stop_daemon(data: ReqData, state: RS) -> impl Responder {
  let mut rtodo = get_rtodo_write_gurad(&state).await;
  if !check_token(&data, &rtodo) {
    return nerr(100, "Invalid token");
  }
  info!("Info: stopping daemon");
  rtodo.stop_daemon();
  std::process::exit(0);
}

pub fn start_server(rtodo: Arc<RwLock<Rtodo>>) -> () {
  {
    match rtodo.write() {
      Ok(data) => {
        #[cfg(debug_assertions)]
        info!(
          "Info: got write lock of works at line:{}, file: {}",
          line!(),
          file!()
        );
        data
      }
      Err(err) => {
        error!(
          "Error: Internal error: {}, line:{}, file: {}",
          err,
          line!(),
          file!()
        );
        return;
      }
    }
    .executor_pid = {
      #[cfg(target_family = "unix")]
      nix::unistd::getpid().as_raw()
    }
  }
  let rt = Runtime::new().unwrap();
  let addr = match rtodo.read().unwrap().config.address.clone() {
    Some(a) => a,
    None => "0.0.0.0:6472".to_string(),
  };
  rt.block_on(async {
    let state = web::Data::new(RtodoState {
      rtodo: rtodo.clone(),
    });
    let server = HttpServer::new(move || {
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
            .route("/validateToken", web::post().to(validate_token))
            .route("/getEntries", web::post().to(get_entries))
            .route("/getWorks", web::post().to(get_works))
            .route("/addEntries", web::post().to(add_entries))
            .route("/deleteEntries", web::post().to(delete_entries))
            .route("/editEntry", web::post().to(edit_entry))
            .route("/stopDaemon", web::post().to(stop_daemon)),
        )
        .service(web::resource("/").route(web::get().to(hello)))
    })
    .bind(&addr)
    .unwrap_or_else(|err| panic!("Error: Failed to bind address: {}, Error: {}", addr, err))
    .run();
    info!("Info: Server started at {}", addr);
    server.await.unwrap_or_else(|err| {
      error!("Error: Server error: {}", err);
    });
  })
}

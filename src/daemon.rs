use crate::server::start_server;
use crate::types::*;
use log::error;
use std::error::Error;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time;

pub fn start_daemon(rtodo_rwl: RwLock<Rtodo>) -> Result<(), Box<dyn Error>> {
  let rtodo_rwl = Arc::new(rtodo_rwl);
  let rtodo_rwl_move = rtodo_rwl.clone();
  thread::spawn(move || start_server(&rtodo_rwl_move));
  thread::spawn(move || loop {
    thread::sleep(time::Duration::from_millis(100));
    let works = match rtodo_rwl.read() {
      Ok(data) => data,
      Err(err) => {
        error!("Error: Internal error: {}", err);
        continue;
      }
    }
    .works
    .clone();
    for work in works.iter() {
      if work.exec_time.is_up() {}
    }
  });
  Ok(())
}

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
  let server_thread = thread::spawn(move || start_server(&rtodo_rwl_move));
  let executor_thread = thread::spawn(move || loop {
    thread::sleep(time::Duration::from_millis(100));
    let works = &match rtodo_rwl.read() {
      Ok(data) => data,
      Err(err) => {
        error!("Error: Internal error: {}", err);
        continue;
      }
    }
    .works;
    for work_rwl in works.iter() {
      let work_read_guard = match work_rwl.read() {
        Ok(data) => data,
        Err(err) => {
          error!("Error: Internal error: {}", err);
          continue;
        }
      };
      if work_read_guard.exec_time.is_up() {
        match work_read_guard.status {
          Status::Running => {
            let mut work_write_guard = match work_rwl.write() {
              Ok(data) => data,
              Err(err) => {
                error!("Error: Internal error: {}", err);
                continue;
              }
            };
            match work_read_guard.entry.do_if_running {
              DoIfRunning::Continue => (),
              DoIfRunning::StartNew => {
                match work_write_guard.start() {
                  Ok(_) => (),
                  Err(err) => {
                    error!("Error: Failed in start entry {}, Error Info: {}", work_read_guard.entry.name, err);
                    work_write_guard.status = Status::Error;
                    continue;
                  }
                }
                ()
              }
              DoIfRunning::Stop => {
                match work_write_guard.stop() {
                  Ok(_) => (),
                  Err(err) => {
                    error!("Error: Failed in stop entry {}, Error Info: {}", work_read_guard.entry.name, err);
                    work_write_guard.status = Status::Error;
                    continue;
                  }
                }
                ()
              }
              DoIfRunning::Restart => {
                match work_write_guard.restart() {
                  Ok(_) => (),
                  Err(err) => {
                    error!("Error: Failed in restart entry {}, Error Info: {}", work_read_guard.entry.name, err);
                    work_write_guard.status = Status::Error;
                    continue;
                  }
                }
                ()
              }
            }
          }
          Status::Paused => (),
          Status::Pending => {
            let mut work_write_guard = match work_rwl.write() {
              Ok(data) => data,
              Err(err) => {
                error!("Error: Internal error: {}", err);
                continue;
              }
            };
            match work_write_guard.start() {
              Ok(_) => (),
              Err(err) => {
                error!("Error: Failed in start entry {}, Error Info: {}", work_read_guard.entry.name, err);
                work_write_guard.status = Status::Error;
                continue;
              }
            }
          }
          Status::Error => ()
        }
      }
    }
  });
  Ok(())
}

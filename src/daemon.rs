use crate::server::start_server;
use crate::types::*;
use crate::utils::check_if_process_by_pid_alive;
use log::{error, info};
use std::error::Error;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time;

pub fn start_executor(rtodo_rwl: Arc<RwLock<Rtodo>>) {
  info!("Info: Starting exectutor");
  loop {
    thread::sleep(time::Duration::from_millis(100));
    let works = &match rtodo_rwl.try_read() {
      Ok(data) => data,
      Err(err) => {
        error!("Error: Internal error: {}", err);
        continue;
      }
    }
    .works;
    for work_rwl in works.iter() {
      let work_read_guard = match work_rwl.try_read() {
        Ok(data) => data,
        Err(err) => {
          error!("Error: Internal error: {}", err);
          continue;
        }
      };
      #[cfg(debug_assertions)]
      print!("{}", work_read_guard.entry.name);
      match &work_read_guard.entry.trigger {
        Trigger::Timer(_) => {
          if work_read_guard.trigger_state.exec_time.clone().unwrap().is_up() {
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
                        error!(
                          "Error: Failed in start entry {}, Error Info: {}",
                          work_read_guard.entry.name, err
                        );
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
                        error!(
                          "Error: Failed in stop entry {}, Error Info: {}",
                          work_read_guard.entry.name, err
                        );
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
                        error!(
                          "Error: Failed in restart entry {}, Error Info: {}",
                          work_read_guard.entry.name, err
                        );
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
                    error!(
                      "Error: Failed in start entry {}, Error Info: {}",
                      work_read_guard.entry.name, err
                    );
                    work_write_guard.status = Status::Error;
                    continue;
                  }
                }
              }
              Status::Error => (),
            }
          }
        }
        Trigger::None => ()
      }
    }
  }
}

pub fn start_checker(rtodo_rwl: Arc<RwLock<Rtodo>>) {
  info!("Info: Starting checker");
  loop {
    thread::sleep(time::Duration::from_millis(100));
    let works = &match rtodo_rwl.try_read() {
      Ok(data) => data,
      Err(err) => {
        error!("Error: Internal error: {}", err);
        continue;
      }
    }
    .works;
    for work_rwl in works.iter() {
      let work_read_guard = match work_rwl.try_read() {
        Ok(data) => data,
        Err(err) => {
          #[cfg(debug_assertions)]
          error!("Error: Internal error: {}", err);
          continue;
        }
      };
      for (index, thread) in work_read_guard.running_processes.iter().enumerate() {
        if !check_if_process_by_pid_alive(thread.pid) {
          match work_rwl.try_write() {
            Ok(data) => data,
            Err(err) => {
              #[cfg(debug_assertions)]
              error!("Error: Internal error: {}", err);
              continue;
            }
          }
          .status = Status::Pending;
        }
      }
    }
  }
}

pub fn start_daemon(rtodo_rwl: RwLock<Rtodo>) -> Result<(), Box<dyn Error>> {
  let rtodo_rwl = Arc::new(rtodo_rwl);
  let rtodo_rwl_move = rtodo_rwl.clone();
  let server_thread = thread::spawn(move || start_server(rtodo_rwl_move));
  let rtodo_rwl_move = rtodo_rwl.clone();
  let executor_thread = thread::spawn(move || start_executor(rtodo_rwl_move));
  let rtodo_rwl_move = rtodo_rwl.clone();
  let checker_thread = thread::spawn(move || start_checker(rtodo_rwl_move));
  server_thread.join().unwrap();
  executor_thread.join().unwrap();
  checker_thread.join().unwrap();
  Ok(())
}

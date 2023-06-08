use crate::server::start_server;
use crate::types::*;
use crate::utils::check_if_process_by_pid_alive;
use ctrlc;
use log::{error, info};
use std::error::Error;
use std::process::exit;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time;

pub fn start_executor(rtodo_rwl: Arc<RwLock<Rtodo>>) {
  {
    match rtodo_rwl.write() {
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
  info!("Info: Starting exectutor");
  loop {
    thread::sleep(time::Duration::from_millis(100));
    let works = {
      &match rtodo_rwl.try_read() {
        Ok(data) => {
          match data.daemon_status {
            RtodoDaemonStatus::Running => (),
            RtodoDaemonStatus::Stopped => {
              info!("Info: Stopping executor");
              exit(0);
            }
          };
          data
        }
        Err(err) => {
          error!(
            "Error: Internal error: {}, line:{}, file: {}",
            err,
            line!(),
            file!()
          );
          continue;
        }
      }
      .works
    };
    for work_rwl in works.iter() {
      let work = {
        match work_rwl.try_read() {
          Ok(data) => data,
          Err(err) => {
            error!(
              "Error: Internal error: {}, line:{}, file: {}",
              err,
              line!(),
              file!()
            );
            continue;
          }
        }
        .clone()
      };
      //#[cfg(debug_assertions)]
      //print!("{}", work_write_guard.entry.name);
      match &work.entry.trigger {
        Trigger::Timer(_) => {
          if work.trigger_state.exec_time.clone().unwrap().is_up() {
            match work.status {
              Status::Running => {
                let mut work_write_guard = match work_rwl.try_write() {
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
                    continue;
                  }
                };
                match work_write_guard.entry.do_if_running {
                  DoIfRunning::Continue => (),
                  DoIfRunning::StartNew => {
                    match work_write_guard.start() {
                      Ok(_) => (),
                      Err(err) => {
                        error!(
                          "Error: Failed in start entry {}, Error Info: {}",
                          work_write_guard.entry.name, err
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
                          work_write_guard.entry.name, err
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
                          work_write_guard.entry.name, err
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
                let mut work_write_guard = match work_rwl.try_write() {
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
                    continue;
                  }
                };
                match work_write_guard.start() {
                  Ok(_) => (),
                  Err(err) => {
                    error!(
                      "Error: Failed in start entry {}, Error Info: {}",
                      work_write_guard.entry.name, err
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
        Trigger::None => (),
      }
    }
  }
}

pub fn start_checker(rtodo_rwl: Arc<RwLock<Rtodo>>) {
  {
    match rtodo_rwl.write() {
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
    .checker_pid = {
      #[cfg(target_family = "unix")]
      nix::unistd::getpid().as_raw()
    }
  }
  info!("Info: Starting checker");
  loop {
    thread::sleep(time::Duration::from_millis(100));
    let works = {
      &match rtodo_rwl.try_read() {
        Ok(data) => data,
        Err(err) => {
          error!(
            "Error: Internal error: {}, line:{}, file: {}",
            err,
            line!(),
            file!()
          );
          continue;
        }
      }
      .works
    };
    for work_rwl in works.iter() {
      let work = {
        match work_rwl.try_read() {
          Ok(data) => data,
          Err(err) => {
            #[cfg(debug_assertions)]
            error!(
              "Error: Internal error: {}, line:{}, file: {}",
              err,
              line!(),
              file!()
            );
            continue;
          }
        }
        .clone()
      };
      for (_, thread) in work.running_processes.iter().enumerate() {
        if !check_if_process_by_pid_alive(thread.pid) {
          match work_rwl.try_write() {
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
              #[cfg(debug_assertions)]
              error!(
                "Error: Internal error: {}, line:{}, file: {}",
                err,
                line!(),
                file!()
              );
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
  ctrlc::set_handler(|| exit(0)).unwrap();
  let rtodo_rwl = Arc::new(rtodo_rwl);
  let rtodo_rwl_move = rtodo_rwl.clone();
  let server_thread = thread::spawn(move || start_server(rtodo_rwl_move));
  let rtodo_rwl_move = rtodo_rwl.clone();
  thread::spawn(move || start_executor(rtodo_rwl_move));
  let rtodo_rwl_move = rtodo_rwl.clone();
  thread::spawn(move || start_checker(rtodo_rwl_move));
  match server_thread.join() {
    Ok(_) => (),
    Err(_) => {
      error!("Error: Failed to join server thread");
    }
  }
  Ok(())
}

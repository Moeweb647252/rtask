use crate::types::*;
use log::info;
#[cfg(target_family = "unix")]
use nix::{sys::signal::kill, unistd::Pid};
use rand::Rng;
use serde::Serialize;
use std::{
  str::FromStr,
  sync::{RwLockReadGuard, RwLockWriteGuard},
};

pub fn generate_token() -> String {
  let mut rng = rand::thread_rng();
  let mut token = String::new();
  for _ in 0..32 {
    let num = rng.gen_range(0..16);
    let c = match num {
      0..=9 => (num + 48) as u8 as char,
      10..=15 => (num + 87) as u8 as char,
      _ => panic!("Invalid number"),
    };
    token.push(c);
  }
  token
}

pub fn check_if_help_in_args(args: &[String]) -> bool {
  for arg in args.iter() {
    if arg == "--help" {
      return true;
    }
  }
  false
}

pub fn check_token(data: &ReqData, rtask: &Rtask) -> bool {
  #[cfg(debug_assertions)]
  info!(
    "Info: token in request: {}, token in rtask: {}",
    data
      .get("token")
      .unwrap_or(&serde_json::Value::Null)
      .as_str()
      .unwrap_or(""),
    rtask.get_token()
  );
  data
    .get("token")
    .unwrap_or(&serde_json::Value::Null)
    .as_str()
    .unwrap_or("")
    == rtask.get_token()
}

pub fn nerr(code: i32, msg: &str) -> String {
  let err = ResCommonData::new(code, msg);
  serde_json::to_string(&err).unwrap()
}

pub fn nsucc<T: Serialize>(code: i32, data: T) -> String {
  let succ = ResCommonData::new(code, data);
  serde_json::to_string_pretty(&succ).unwrap()
}

pub fn garg<T: FromStr>(args: &[String], index: usize) -> Option<T> {
  let err = "Invalid argument";
  match (match args.get(index).ok_or(err) {
    Ok(data) => data,
    Err(_) => {
      return None;
    }
  })
  .clone()
  .parse::<T>()
  {
    Ok(data) => Some(data),
    Err(_) => None,
  }
}

pub fn random_name() -> String {
  "Not impled".to_string()
}

#[cfg(target_family = "unix")]
pub fn check_if_process_by_pid_alive(pid: i32) -> bool {
  match kill(Pid::from_raw(pid), None) {
    Ok(_) => true,
    Err(_) => false,
  }
}

pub async fn get_rtask_read_gurad(state: &RS) -> RwLockReadGuard<Rtask> {
  loop {
    match state.rtask.try_read() {
      Ok(data) => break data,
      Err(_) => tokio::time::sleep(tokio::time::Duration::from_millis(100)).await,
    }
  }
}

pub async fn get_rtask_write_gurad(state: &RS) -> RwLockWriteGuard<Rtask> {
  loop {
    match state.rtask.try_write() {
      Ok(data) => {
        #[cfg(debug_assertions)]
        info!(
          "Info: got write lock of works at line:{}, file: {}",
          line!(),
          file!()
        );
        break data;
      }
      Err(_) => tokio::time::sleep(tokio::time::Duration::from_millis(100)).await,
    }
  }
}

use rand::Rng;
use serde::Serialize;

use crate::types::*;

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

pub fn check_if_help_in_args(args: &Vec<String>) -> bool {
  for arg in args.iter() {
    if arg == "--help" {
      return true;
    }
  }
  false
}

pub fn check_token(data: &ReqData, rtodo: &Rtodo) -> bool {
  data
    .get("token")
    .unwrap_or(&serde_json::Value::Null)
    .as_str()
    .unwrap_or("")
    == rtodo.get_token()
}

pub fn nerr(code: i32, msg: &str) -> String {
  let err = Err::new(code, msg);
  serde_json::to_string(&err).unwrap()
}

pub fn nsucc<T: Serialize>(code: i32, data: T) -> String {
  let succ = Succ::new(code, data);
  serde_json::to_string(&succ).unwrap()
}

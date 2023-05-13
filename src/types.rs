use actix_web::web;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

pub type RS = web::Data<RtodoState>;
pub type ReqData = web::Json<serde_json::Value>;

#[derive(Serialize, Deserialize)]
pub struct Err {
  pub code: i32,
  pub msg: String,
}

#[derive(Serialize, Deserialize)]
pub struct Succ<T: Serialize> {
  pub code: i32,
  pub data: T,
}

pub struct RtodoState {
  pub rtodo: RwLock<Rtodo>,
}

pub struct Rtodo {
  pub config: Config,
  pub cur_entry_id: i32,
  pub conf_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub entries: Vec<Entry>,
  pub address: String,
  pub token: String,
}

pub enum Status {
  Running,
  Paused,
  Pending,
}

#[derive(Serialize, Deserialize)]
pub struct Work {
  pub status: i32,
  pub entry: Entry,
}

pub enum EntryIdentifier {
  Id(u32),
  Name(String),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Log {
  File(String),
  Default,
  Off,
}

impl Default for Log {
  fn default() -> Self {
    Log::Default
  }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Time {
  pub second: u32,
  pub minute: u32,
  pub hour: u32,
  pub day: u32,
  pub month: u32,
  pub year: u32,
}
#[derive(Serialize, Deserialize, Clone)]
pub enum Timer {
  Repeat(Time),
  Once(Time),
  ManyTimes(Time, u32),
  None,
}

impl Default for Timer {
  fn default() -> Self {
    Timer::None
  }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Entry {
  pub id: i32,
  pub name: Option<String>,
  pub command: String,
  pub env: Option<Vec<String>>,
  pub working_dir: Option<String>,
  pub log: Log,
  pub timer: Timer,
}

pub enum OperationType {
  Add,
  Delete,
  Start,
  Pause,
  StartDaemon,
  StopDaemon,
  List,
  Detail,
  Help,
  Version,
}

pub enum Operation {
  Add(Entry),
  Delete(EntryIdentifier),
  Start(EntryIdentifier),
  Pause(EntryIdentifier),
  StartDaemon(),
  StopDaemon(),
  List(),
  Detail(EntryIdentifier),
  Help(Option<OperationType>),
  Version,
}

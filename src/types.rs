use actix_web::web;
use serde::{Deserialize, Serialize};
use std::sync::{RwLock, Arc};

pub type RS = web::Data<RtodoState>;
pub type ReqData = web::Json<serde_json::Value>;
pub type ReqDataT<T> = web::Json<ReqCommonData<T>>;

#[derive(Serialize, Deserialize, Clone)]
pub struct ReqCommonData<T> {
  pub token: String,
  pub data: Option<T>
}

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
  pub rtodo: Arc<RwLock<Rtodo>>,
}

pub struct Rtodo {
  pub config: Config,
  pub cur_entry_id: i32,
  pub conf_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
  #[serde(default)]
  pub entries: Vec<Entry>,
  pub address: Option<String>,
  pub token: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Status {
  Running,
  Paused,
  Pending,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum TimeZone {
  UTC,
  Local,
  Offset(i8),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DateTime {
  pub sec: u64,
  pub min: u64,
  pub hour: u64,
  pub day: u64,
  pub month: u64,
  pub year: u64,
  pub time_zone: TimeZone
}

impl Default for DateTime {
  fn default() -> Self {
    Self { sec: 0, min: 0, hour: 0, day: 0, month: 0, year: 0, time_zone: TimeZone::Local }
  }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Duration {
  pub sec: u64,
  pub min: u64,
  pub hour: u64,
  pub day: u64,
  pub month: u64,
  pub year: u64,
  pub total_sec: u64,
}

impl Default for Duration {
  fn default() -> Self {
      Self { sec: 0, min: 0, hour: 0, day: 0, month: 0, year: 0, total_sec: 0 }
  }
}

#[derive(Serialize, Deserialize)]
pub struct Work {
  pub status: Status,
  pub entry: Entry,
  pub last_exec_time: DateTime
}

pub enum EntryIdentifier {
  Id(u32),
  Name(String),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Logger {
  File(String),
  Default,
  Off,
}

impl Default for Logger {
  fn default() -> Self {
    Self::Default
  }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Timer {
  Repeat(Duration),
  Once(DateTime),
  ManyTimes(Duration, u32),
  Never,
}

impl Default for Timer {
  fn default() -> Self {
    Timer::Never
  }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Action {
  Command(String),
  None,
}

impl Default for Action {
  fn default() -> Self {
    Action::None
  }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Entry {
  pub id: i32,
  pub name: Option<String>,
  pub action: Option<Action>,
  pub env: Option<Vec<String>>,
  pub working_dir: Option<String>,
  pub logger: Logger,
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

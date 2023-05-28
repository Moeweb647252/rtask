use actix_web::web;
use serde::{Deserialize, Serialize};
use std::{
  path::PathBuf,
  sync::{Arc, RwLock}, collections::HashMap,
};

pub type RS = web::Data<RtodoState>;
pub type ReqData = web::Json<serde_json::Value>;
pub type ReqDataT<T> = web::Json<ReqCommonData<T>>;

#[derive(Debug)]
pub struct RtodoError {
  pub msg: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ReqCommonData<T> {
  pub token: String,
  pub data: Option<T>,
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
  pub works: Vec<RwLock<Work>>,
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
  Error,
  Running,
  Paused,
  Pending,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum TimeZone {
  Utc,
  Local,
  Offset(i8),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DateTime {
  pub year: i32,
  pub month: u32,
  pub day: u32,
  pub hour: u32,
  pub min: u32,
  pub sec: u32,
  pub timestamp: i64,
  pub time_zone: TimeZone,
}

impl Default for DateTime {
  fn default() -> Self {
    Self {
      sec: 0,
      min: 0,
      hour: 0,
      day: 0,
      month: 0,
      year: 0,
      timestamp: 0,
      time_zone: TimeZone::Local,
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Duration {
  pub year: i32,
  pub month: u32,
  pub day: u32,
  pub hour: u32,
  pub min: u32,
  pub sec: u32,
  pub total_sec: u64,
}

#[derive(Serialize, Clone)]
pub struct Process {
  pub pid: u32,
  pub output_tmp_file: Option<PathBuf>,
}

#[derive(Serialize, Clone)]
pub struct Work {
  pub status: Status,
  pub entry: Entry,
  pub exec_time: DateTime,
  pub exec_times: u32,
  pub running_processes: Vec<Process>,
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

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum Timer {
  Repeat(Duration),
  Once(DateTime),
  ManyTimes(Duration, u32),
  #[default]
  Never,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UnixUser {
  pub uid: u32,
  pub gid: u32,
  pub username: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WindowsUser {
  pub username: String,
  pub group_windows: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SystemUser {
  Unix(UnixUser),
  Windows(WindowsUser),
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Execute {
  pub env: Option<HashMap<String, String>>,
  pub working_dir: Option<String>,
  pub executable: PathBuf,
  pub user: Option<SystemUser>,
  pub args: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum Action {
  Exec(Execute),
  #[default]
  None,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum DoIfRunning {
    #[default]
    StartNew,
    Stop,
    Restart,
    Continue,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Entry {
  pub id: i32,
  pub name: String,
  pub action: Action,
  pub logger: Logger,
  pub timer: Timer,
  pub do_if_running: DoIfRunning,
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

pub trait CommandHelp {
    fn cmd_help() -> String;
}
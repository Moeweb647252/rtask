use crate::types::*;
use crate::utils::*;
use chrono::TimeZone;
use chrono::{Datelike, Timelike};
use log::{error, info};
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::ops;
use std::path::PathBuf;
use std::process;
use std::sync::RwLock;
use sysinfo::{SystemExt, UserExt};

impl Operation {
  pub fn from_args(args: &[String]) -> Result<Operation, Box<dyn Error>> {
    let mut operation: Operation = Operation::Help(None);
    let op_str = args[1].clone();
    match op_str.as_str() {
      "add" => {
        if check_if_help_in_args(args) {
          return Ok(Operation::Help(Some(OperationType::Add)));
        }
        let entry = Entry::from_args(
          args,
          Trigger::from_args(args),
          Logger::from_args(args),
          Action::from_args(args),
          DoIfRunning::from_args(args),
          Status::from_args(args),
        )?;
        operation = Operation::Add(entry);
      }
      "delete" => {
        if check_if_help_in_args(args) {
          return Ok(Operation::Help(Some(OperationType::Delete)));
        }
        if let Ok(id) = args[2].parse::<u32>() {
          operation = Operation::Delete(EntryIdentifier::Id(id));
        } else if let Ok(name) = args[2].parse::<String>() {
          operation = Operation::Delete(EntryIdentifier::Name(name));
        } else {
          return Err("Invalid entry identifier".into());
        }
      }
      "start" => {
        if check_if_help_in_args(args) {
          return Ok(Operation::Help(Some(OperationType::Start)));
        }
        if let Ok(id) = args[2].parse::<u32>() {
          operation = Operation::Start(EntryIdentifier::Id(id));
        } else if let Ok(name) = args[2].parse::<String>() {
          operation = Operation::Start(EntryIdentifier::Name(name));
        } else {
          return Err("Invalid entry identifier".into());
        }
      }
      "pause" => {
        if check_if_help_in_args(args) {
          return Ok(Operation::Help(Some(OperationType::Pause)));
        }
        if let Ok(id) = args[2].parse::<u32>() {
          operation = Operation::Pause(EntryIdentifier::Id(id));
        } else if let Ok(name) = args[2].parse::<String>() {
          operation = Operation::Pause(EntryIdentifier::Name(name));
        } else {
          return Err("Invalid entry identifier".into());
        }
      }
      "start-daemon" => {
        operation = Operation::StartDaemon();
      }
      "stop-daemon" => {
        operation = Operation::StopDaemon();
      }
      "list" => {
        operation = Operation::List();
      }
      "detail" => {
        if check_if_help_in_args(args) {
          return Ok(Operation::Help(Some(OperationType::Detail)));
        }
        if let Ok(id) = args[2].parse::<u32>() {
          operation = Operation::Detail(EntryIdentifier::Id(id));
        } else if let Ok(name) = args[2].parse::<String>() {
          operation = Operation::Detail(EntryIdentifier::Name(name));
        } else {
          return Err("Invalid entry identifier".into());
        }
      }
      "help" => {
        if check_if_help_in_args(args) {
          return Ok(Operation::Help(Some(OperationType::Help)));
        }
        operation = Operation::Help(None);
      }
      "version" => {
        operation = Operation::Version;
      }
      _ => {}
    }
    Ok(operation)
  }
}

impl Entry {
  pub fn new(
    trigger: Trigger,
    logger: Logger,
    action: Action,
    do_if_running: DoIfRunning,
    status: Status,
  ) -> Self {
    Self {
      id: 0,
      name: String::new(),
      action,
      logger,
      trigger,
      status,
      do_if_running,
    }
  }
  pub fn from_args(
    args: &[String],
    trigger: Trigger,
    logger: Logger,
    action: Action,
    do_if_running: DoIfRunning,
    status: Status,
  ) -> Result<Self, Box<dyn Error>> {
    let mut entry = Self::new(trigger, logger, action, do_if_running, status);
    let err = "Invalid argument";
    entry.name = random_name();
    for (index, arg) in args.iter().enumerate() {
      match arg.as_str() {
        "--name" => {
          let name = args.get(index + 1).ok_or(err)?.clone();
          entry.name = name;
        }
        "--some" => (),
        _ => (),
      }
    }
    Ok(entry)
  }
}

impl Logger {
  pub fn from_args(args: &[String]) -> Self {
    let mut logger = Self::Default;
    for (index, arg) in args.iter().enumerate() {
      match arg.as_str() {
        "--log-file" => {
          logger = match garg(args, index + 1) {
            Some(data) => Logger::File(data),
            None => Logger::Off,
          }
        }
        "--log-off" => logger = Logger::Off,
        _ => (),
      }
    }
    logger
  }
}

impl Config {
  pub fn add_entry(&mut self, entry: &Entry, id: i32) {
    let mut entry = entry.clone();
    entry.id = id;
    self.entries.push(entry);
  }
}

impl Default for Config {
  fn default() -> Self {
    Self {
      entries: Vec::new(),
      address: Some(String::from("0.0.0.0:6472")),
      token: generate_token(),
    }
  }
}

impl Rtodo {
  pub fn add_entry(&mut self, entry: Entry) {
    self.config.add_entry(&entry, self.cur_entry_id + 1);
  }

  pub fn write_conf(&self) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(&self.conf_path)?;
    let content = serde_json::to_string(&self.config)?;
    file.write_all(content.as_bytes())?;
    Ok(())
  }

  pub fn get_entries(&self) -> Vec<Entry> {
    self.config.entries.clone()
  }

  pub fn get_token(&self) -> &str {
    self.config.token.as_str()
  }

  pub fn init_works(&mut self) -> Result<(), Box<dyn Error>> {
    for entry in self.get_entries() {
      self.works.push(RwLock::new(Work {
        status: entry.status,
        entry: entry.clone(),
        trigger_state: TriggerState::from_entry(&entry),
        running_processes: Vec::new(),
      }))
    }
    Ok(())
  }
}

impl Err {
  pub fn new(code: i32, msg: &str) -> Self {
    Self {
      code,
      msg: msg.to_string(),
    }
  }
}

impl<T: Serialize> Succ<T> {
  pub fn new(code: i32, data: T) -> Self {
    Self { code, data }
  }
}

impl<T> ReqCommonData<T> {
  pub fn check_token(&self, rtodo: &Rtodo) -> bool {
    self.token == rtodo.get_token()
  }
}

impl DateTime {
  pub fn from_ymd_hms(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
    sec: u32,
  ) -> Option<Self> {
    let new = chrono::Local.with_ymd_and_hms(year, month, day, hour, min, sec);
    let new = match new.single() {
      Some(data) => data,
      None => {
        error!("chrono::Local.with_ymd_and_hms::single returns None!");
        return None
      }
        ,
    };
    Some(Self {
      sec: new.second(),
      min: new.minute(),
      hour: new.hour(),
      day: new.day(),
      month: new.month(),
      year: new.year(),
      timestamp: new.timestamp(),
      time_zone: crate::types::TimeZone::Local,
    })
  }

  pub fn from_args(args: &[String]) -> Option<Self> {
    let mut hasarg = false;
    let mut datetime = Self::default();
    for (index, arg) in args.iter().enumerate() {
      match arg.as_str() {
        "--sec" => {
          hasarg = true;
          datetime.sec = match garg(args, index + 1) {
            Some(data) => data,
            None => {
              continue;
            }
          }
        }
        "--min" => {
          hasarg = true;
          datetime.min = match garg(args, index + 1) {
            Some(data) => data,
            None => {
              continue;
            }
          }
        }
        "--hour" => {
          hasarg = true;
          datetime.hour = match garg(args, index + 1) {
            Some(data) => data,
            None => {
              continue;
            }
          }
        }
        "--day" => {
          hasarg = true;
          datetime.day = match garg(args, index + 1) {
            Some(data) => data,
            None => {
              continue;
            }
          }
        }
        "--month" => {
          hasarg = true;
          datetime.month = match garg(args, index + 1) {
            Some(data) => data,
            None => {
              continue;
            }
          }
        }
        "--year" => {
          hasarg = true;
          datetime.year = match garg(args, index + 1) {
            Some(data) => data,
            None => {
              continue;
            }
          }
        }
        _ => (),
      }
    }
    if hasarg {
      Some(datetime)
    } else {
      None
    }
  }

  pub fn from_duration(duration: &Duration) -> Option<Self> {
    Self::now() + duration.clone()
  }

  pub fn now() -> Self {
    let now = chrono::Local::now();
    Self {
      sec: now.second(),
      min: now.minute(),
      hour: now.hour(),
      day: now.day(),
      month: now.month(),
      year: now.year(),
      timestamp: now.timestamp(),
      time_zone: crate::types::TimeZone::Local,
    }
  }
  pub fn one_day() -> Self {
    let now = chrono::Local::now();
    Self {
      sec: now.second(),
      min: now.minute(),
      hour: now.hour(),
      day: (now.day() + 1),
      month: now.month(),
      year: now.year(),
      timestamp: now.timestamp() + chrono::Duration::days(1).num_seconds(),
      time_zone: crate::types::TimeZone::Local,
    }
  }

  pub fn timestamp(&self) -> i64 {
    chrono::Local
      .with_ymd_and_hms(
        self.year, self.month, self.day, self.hour, self.min, self.sec,
      )
      .unwrap()
      .timestamp()
  }

  pub fn is_up(&self) -> bool {
    self.timestamp() >= chrono::Local::now().timestamp()
  }
}

impl ops::Add<Duration> for DateTime {
  type Output = Option<DateTime>;
  fn add(self, duration: Duration) -> Self::Output {
    Self::from_ymd_hms(
      self.year + duration.year,
      self.month + duration.month,
      self.day + duration.day,
      self.hour + duration.hour,
      self.min + duration.min,
      self.sec + duration.sec,
    )
  }
}

impl Duration {
  fn from_args(args: &[String]) -> Option<Self> {
    let mut hasarg = false;
    let mut duration = Self::default();
    for (index, arg) in args.iter().enumerate() {
      match arg.as_str() {
        "--sec" => {
          hasarg = true;
          duration.sec = match garg(args, index + 1) {
            Some(data) => data,
            None => {
              continue;
            }
          }
        }
        "--min" => {
          hasarg = true;
          duration.min = match garg(args, index + 1) {
            Some(data) => data,
            None => {
              continue;
            }
          }
        }
        "--hour" => {
          hasarg = true;
          duration.hour = match garg(args, index + 1) {
            Some(data) => data,
            None => {
              continue;
            }
          }
        }
        "--day" => {
          hasarg = true;
          duration.day = match garg(args, index + 1) {
            Some(data) => data,
            None => {
              continue;
            }
          }
        }
        "--month" => {
          hasarg = true;
          duration.month = match garg(args, index + 1) {
            Some(data) => data,
            None => {
              continue;
            }
          }
        }
        "--year" => {
          hasarg = true;
          duration.year = match garg(args, index + 1) {
            Some(data) => data,
            None => {
              continue;
            }
          }
        }
        _ => (),
      }
    }
    if hasarg {
      Some(duration)
    } else {
      None
    }
  }
  pub fn one_day() -> Self {
    Self {
      sec: 0,
      min: 0,
      hour: 0,
      day: 1,
      month: 0,
      year: 0,
      total_sec: 3600,
    }
  }
}

impl Timer {
  fn from_args(args: &[String]) -> Option<Self> {
    let mut timer = Self::default();
    let mut hasarg = false;
    for arg in args.iter() {
      match arg.as_str() {
        "--repeat" => {
          hasarg = true;
          timer = Self::Repeat(match Duration::from_args(args) {
            Some(data) => data,
            None => Duration::one_day(),
          })
        }
        "--once" => {
          hasarg = true;
          timer = Self::Once(match DateTime::from_args(args) {
            Some(data) => data,
            None => DateTime::one_day(),
          })
        }
        "--never" => timer = Self::Never,
        _ => (),
      }
    }
    if hasarg {
      Some(timer)
    } else {
      None
    }
  }
}

impl Execute {
  pub fn from_args(args: &[String]) -> Option<Self> {
    let mut execute = Self::default();
    let mut hasarg = false;
    for (index, arg) in args.iter().enumerate() {
      match arg.as_str() {
        "--exec" => {
          execute.executable = PathBuf::from(match garg::<String>(args, index + 1) {
            Some(data) => data,
            None => continue,
          });
          hasarg = true;
        }
        "--env" => {
          execute.env = Some(
            (match garg::<String>(args, index + 1) {
              Some(data) => data,
              None => continue,
            })
            .split_whitespace()
            .filter_map(|pair| pair.split_once("="))
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect(),
          );
        }
        "--args" => {
          execute.args = match garg::<String>(args, index + 1) {
            Some(data) => Some(data.split(' ').map(|s| s.to_string()).collect()),
            None => None,
          }
        }
        "--dir" => execute.working_dir = garg(args, index + 1),
        "--username" => execute.user = SystemUser::from_un(garg(args, index + 1)),
        _ => (),
      }
    }
    if hasarg {
      Some(execute)
    } else {
      None
    }
  }

  pub fn exec(&self) -> Result<u32, Box<dyn Error>> {
    let child = process::Command::new(self.executable.clone())
      .args(self.args.clone().unwrap_or(vec![]))
      .envs(self.env.clone().unwrap_or(HashMap::new()))
      .current_dir(self.working_dir.clone().unwrap_or("/tmp".into()))
      .spawn()?;
    Ok(child.id())
  }
}

impl Action {
  pub fn from_args(args: &[String]) -> Self {
    let mut action = Self::default();
    let mut hasarg = false;
    for arg in args {
      match arg.as_str() {
        "--exec" => {
          action = Action::Exec(match Execute::from_args(args) {
            Some(data) => data,
            None => continue,
          });
          hasarg = true;
        }
        "--some" => (),
        _ => (),
      }
    }
    if hasarg {
      action
    } else {
      Self::None
    }
  }
}

impl SystemUser {
  pub fn from_un(un: Option<String>) -> Option<Self> {
    let un = match un {
      Some(data) => data,
      None => return None,
    };
    let info = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::new().with_users_list());
    for user in info.users().iter() {
      if user.name() == un {
        return Some(match env::consts::OS {
          "linux" => Self::Unix(UnixUser {
            uid: **user.id(),
            gid: *user.group_id(),
            username: user.name().to_string(),
          }),
          _ => return None,
        });
      }
    }
    None
  }
}

impl Work {
  pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
    info!("Info: Starting entry: {}", self.entry.name);
    match &self.entry.action {
      Action::Exec(execute) => {
        match self.entry.trigger.clone() {
          Trigger::Timer(timer) => match timer {
            Timer::Repeat(timer) => {
              self.trigger_state.exec_time =
                match self.trigger_state.exec_time.clone().unwrap() + timer {
                  Some(data) => Some(data),
                  None => return Err(RtodoError::new("Error: Invalid time").into()),
                };
              self.trigger_state.exec_times += 1;
              execute.exec()?;
            }
            Timer::Once(_) => {
              if self.trigger_state.exec_times >= 1 {
                return Err(
                  RtodoError::new(
                    format!(
                      "Error: Entry {} with Once timer executed twice!",
                      self.entry.name
                    )
                    .as_str(),
                  )
                  .into(),
                );
              }
              self.trigger_state.exec_times += 1;
              self.status = Status::Paused;
              execute.exec()?;
            }
            Timer::ManyTimes(timer, times) => {
              if self.trigger_state.exec_times >= times {
                return Err(
                  RtodoError::new(
                    format!(
                      "Error: Entry {} with ManyTimes timer executed exceeded times!",
                      self.entry.name
                    )
                    .as_str(),
                  )
                  .into(),
                );
              }
              self.trigger_state.exec_time =
                match self.trigger_state.exec_time.clone().unwrap() + timer {
                  Some(data) => Some(data),
                  None => return Err(RtodoError::new("Error: Invalid time").into()),
                };
              self.trigger_state.exec_times += 1;
              execute.exec()?;
            }
            Timer::Never => {
              return Err(
                RtodoError::new(
                  format!(
                    "Error: Entry with a Never Timer executed! Entry: {}",
                    self.entry.name
                  )
                  .as_str(),
                )
                .into(),
              )
            }
          },
          Trigger::None => {
            error!("Error: Entry {} executed without trigger!", self.entry.name)
          }
        }
        self.status = Status::Running;
      }
      Action::None => (),
    }
    info!("Info: Started entry: {}", self.entry.name);
    Ok(())
  }
  pub fn stop(&mut self) -> Result<(), Box<dyn Error>> {
    info!("Info: Stopping entry: {}", self.entry.name);
    match self.entry.action {
      Action::Exec(_) => {
        for i in &self.running_processes {
          i.kill()?;
        }
        self.running_processes.clear();
        self.status = Status::Paused;
        Ok(())
      }
      Action::None => Ok(()),
    }
  }
  pub fn restart(&mut self) -> Result<(), Box<dyn Error>> {
    self.stop()?;
    self.start()
  }
}

impl Display for RtodoError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

impl Error for RtodoError {}

impl RtodoError {
  pub fn new(msg: &str) -> Self {
    Self {
      msg: msg.to_string(),
    }
  }
}

impl DoIfRunning {
  pub fn from_args(args: &[String]) -> Self {
    let mut do_if_running = Self::default();
    for arg in args {
      match arg.as_str() {
        "--rest-ir" => do_if_running = Self::Restart,
        "--stop-ir" => do_if_running = Self::Stop,
        "--cont-ir" => do_if_running = Self::Continue,
        "--stne-ir" => do_if_running = Self::StartNew,
        _ => (),
      }
    }
    do_if_running
  }
}

impl CommandHelp for DoIfRunning {
  fn cmd_help() -> String {
    String::from(
      "--rest-ir: Restart if work is running
--stop-ir: Stop if work is running
--cont-ir: Continue if work is running
--stne-ir: Start new if work is running
",
    )
  }
}

impl Process {
  pub fn kill(&self) -> Result<(), Box<dyn Error>> {
    process::Command::new("kill")
      .arg("-9")
      .arg(self.pid.to_string())
      .spawn()?;
    Ok(())
  }
}

impl Status {
  pub fn from_args(args: &[String]) -> Self {
    for arg in args {
      match arg.as_str() {
        "--paused" => return Self::Paused,
        _ => (),
      }
    }
    Self::default()
  }
}

impl Trigger {
  pub fn from_args(args: &[String]) -> Self {
    if let Some(timer) = Timer::from_args(args) {
      Self::Timer(timer)
    } else {
      Self::None
    }
  }
}

impl TriggerState {
  pub fn from_entry(entry: &Entry) -> Self {
    match &entry.trigger {
      Trigger::Timer(timer) => match timer {
        Timer::Repeat(timer) => Self {
          exec_time: match DateTime::from_duration(&timer) {
            Some(data) => Some(data),
            None => {
              error!(
                "Error: Repeat timer construct failed from duration at entry {}",
                entry.name
              );
              None
            }
          },
          exec_times: 0,
        },
        Timer::ManyTimes(timer, _) => Self {
          exec_time: match DateTime::from_duration(&timer) {
            Some(data) => Some(data),
            None => {
              error!(
                "Error: Repeat timer construct failed from duration at entry {}",
                entry.name
              );
              None
            }
          },
          exec_times: 0,
        },
        Timer::Once(timer) => Self {
          exec_time: Some(timer.clone()),
          exec_times: 0,
        },
        Timer::Never => Self::default(),
      },
      Trigger::None => Self::default(),
    }
  }
}

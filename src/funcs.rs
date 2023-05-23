use crate::types::*;
use crate::utils::*;
use chrono::TimeZone;
use chrono::{Datelike, Timelike};
use serde::Serialize;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
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
        let time = Timer::from_args(args);

        let entry = Entry::from_args(args, time, Logger::from_args(args)?)?;

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
  pub fn new(timer: Timer, logger: Logger) -> Self {
    Self {
      id: 0,
      name: None,
      action: None,
      logger,
      timer,
    }
  }
  pub fn from_args(args: &[String], timer: Timer, logger: Logger) -> Result<Self, Box<dyn Error>> {
    let mut entry = Self::new(timer, logger);
    let err = "Invalid argument";
    for (index, arg) in args.iter().enumerate() {
      match arg.as_str() {
        "--name" => {
          let name = args.get(index + 1).ok_or(err)?.clone();
          entry.name = Some(name);
        }
        _ => (),
      }
    }
    Ok(entry)
  }
}

impl Logger {
  pub fn from_args(args: &[String]) -> Result<Self, Box<dyn Error>> {
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
    Ok(logger)
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
    let content = serde_yaml::to_string(&self.config)?;
    file.write_all(content.as_bytes())?;
    Ok(())
  }

  pub fn get_entries(&self) -> Vec<Entry> {
    self.config.entries.clone()
  }

  pub fn get_token(&self) -> &str {
    self.config.token.as_str()
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
  fn from_args(args: &[String]) -> Option<Self> {
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
  fn from_args(args: &[String]) -> Self {
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
        _ => (),
      }
    }
    if hasarg {
      timer
    } else {
      Self::Never
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
            .split(',')
            .map(|s| s.to_string())
            .collect(),
          );
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
}

impl Action {
  pub fn from_args(args: &[String]) -> Self {
    let action = Self::default();
    Self::None
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

use crate::types::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Write;

use crate::utils::*;

impl Operation {
  pub fn from_args(args: &Vec<String>) -> Result<Operation, Box<dyn Error>> {
    let mut operation: Operation = Operation::Help(None);
    let op_str = args[1].clone();
    match op_str.as_str() {
      "add" => {
        if check_if_help_in_args(args) {
          return Ok(Operation::Help(Some(OperationType::Add)));
        }
        let time = Time::from_args(args)?;
        let mut entry = Entry::from_args(args, &time)?;

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

impl Time {
  pub fn from_args(args: &Vec<String>) -> Result<Time, Box<dyn Error>> {
    let mut time: Time = Time::default();
    let mut hasarg = false;
    let err = "Invalid argument";
    for (index, arg) in args.iter().enumerate() {
      match arg.as_str() {
        "-d" => {
          hasarg = true;
          let day = args.get(index + 1).ok_or(err)?.parse::<u32>()?;
          time.day = day;
        }
        "-M" => {
          hasarg = true;
          let month = args.get(index + 1).ok_or(err)?.parse::<u32>()?;
          time.month = month;
        }
        "-y" => {
          hasarg = true;
          let year = args.get(index + 1).ok_or(err)?.parse::<u32>()?;
          time.year = year;
        }
        "-h" => {
          hasarg = true;
          let hour = args.get(index + 1).ok_or(err)?.parse::<u32>()?;
          time.hour = hour;
        }
        "-m" => {
          hasarg = true;
          let minute = args.get(index + 1).ok_or(err)?.parse::<u32>()?;
          time.minute = minute;
        }
        "-s" => {
          hasarg = true;
          let second = args.get(index + 1).ok_or(err)?.parse::<u32>()?;
          time.second = second;
        }
        _ => (),
      }
    }
    if hasarg {
      Ok(time)
    } else {
      Err("No time argument".into())
    }
  }
}

impl Entry {
  pub fn from_args(args: &Vec<String>, time: &Time) -> Result<Self, Box<dyn Error>> {
    let mut entry: Self = Self::default();
    let err = "Invalid argument";
    for (index, arg) in args.iter().enumerate() {
      match arg.as_str() {
        "-c" => {
          let command = args.get(index + 1).ok_or(err)?.clone();
          entry.command = command;
        }
        "--once" => {
          entry.timer = Timer::Once(time.clone());
        }
        "--repeat" => {
          entry.timer = Timer::Repeat(time.clone());
        }
        "--mt" => {
          let times = args.get(index + 1).ok_or(err)?.parse::<u32>()?;
          entry.timer = Timer::ManyTimes(time.clone(), times);
        }
        "-n" => {
          let name = args.get(index + 1).ok_or(err)?.clone();
          entry.name = Some(name);
        }
        "--env" => {
          let env = args.get(index + 1).ok_or(err)?.clone();
          entry.env = Some(env.split(",").map(|s| s.to_string()).collect());
        }
        _ => (),
      }
    }
    Ok(entry)
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
      address: String::from("0.0.0.0:6472"),
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

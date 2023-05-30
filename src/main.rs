use std::env::args;
use std::fs;
use std::sync::{RwLock};
use log::{error, info};

mod daemon;
mod funcs;
mod server;
mod types;
mod utils;
use types::*;

fn main() {
  env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
  let args: Vec<String> = args().collect();
  let mut path = std::path::PathBuf::from("/etc/rtodo/rtodo.conf");
  if args.len() < 2 {
    println!("Usage: {} <operation> [args]", args[0]);
    return;
  }
  for (index, arg) in args.iter().enumerate() {
    if arg == "--conf" {
      if let Some(p) = args.get(index + 1) {
        path = std::path::PathBuf::from(p);
      }
    }
  }
  let opt = match Operation::from_args(&args) {
    Ok(opt) => opt,
    Err(err) => {
      err.source().unwrap();
      return;
    }
  };
  let config_content = match fs::read(&path) {
    Ok(content) => content,
    Err(err) => {
      error!("Error: cannot read config file: {}, Err: {}", path.to_str().unwrap_or("Unknown"), err);
      match fs::write(&path, "") {
        Ok(_) => {
          info!("Info: auto created default config file.");
          vec![]
        }
        Err(err) => panic!("Error: cannot create config file: {}, Err: {}", path.to_str().unwrap_or("Unknown"), err)
      }
    }
  };
  let config: Config = match serde_yaml::from_str(&String::from_utf8_lossy(&config_content)) {
    Ok(config) => config,
    Err(err) => {
      if err.to_string().starts_with("missing field") {
        Config::default()
      } else {
        panic!("{}", err);
      }
    }
  };
  let cur_entry_id = config.entries.iter().map(|i| i.id).max().unwrap_or(0);
  let mut rtodo = Rtodo {
    conf_path: path.to_str().unwrap().to_string(),
    works: Vec::new(),
    config,
    cur_entry_id,
  };
  match opt {
    Operation::Add(entry) => {
      let name = entry.name.clone();
      rtodo.add_entry(entry);
      print!("Succussfully added {}", name);
    }
    Operation::StartDaemon() => match daemon::start_daemon(RwLock::new(rtodo)) {
      Ok(_) => {
        return;
      }
      Err(err) => {
        panic!("{}", err);
      }
    },
    _other => (),
  }
  match rtodo.write_conf() {
    Ok(_) => (),
    Err(err) => {
      panic!("{}", err);
    }
  }
}

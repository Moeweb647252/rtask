use log::{error, info};
use std::env::args;
use std::fs;

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
  let mut opt = match Operation::from_args(&args) {
    Ok(opt) => opt,
    Err(err) => {
      err.source().unwrap();
      return;
    }
  };
  let config_content = match fs::read(&path) {
    Ok(content) => content,
    Err(err) => {
      error!(
        "Error: cannot read config file: {}, Err: {}",
        path.to_str().unwrap_or("Unknown"),
        err
      );
      match fs::write(&path, "") {
        Ok(_) => {
          info!("Info: auto created default config file.");
          serde_json::to_string_pretty(&Config::default())
            .unwrap()
            .into()
        }
        Err(err) => panic!(
          "Error: cannot create config file: {}, Err: {}",
          path.to_str().unwrap_or("Unknown"),
          err
        ),
      }
    }
  };
  let config: Config = match serde_json::from_str(&String::from_utf8_lossy(&config_content)) {
    Ok(config) => config,
    Err(err) => {
      if err.to_string().starts_with("missing field") {
        Config::default()
      } else {
        error!(
          "Error: cannot parse config file: {}, Err: {}",
          path.to_str().unwrap_or("Unknown"),
          err
        );
        return ();
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
  rtodo.init_works().unwrap();
  opt.handle(rtodo);
}

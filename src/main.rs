use std::env::args;
use std::fs;

mod daemon;
mod funcs;
mod types;
mod utils;
use types::*;

fn main() {
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
      println!("{}", err);
      return;
    }
  };
  let config_content = match fs::read(&path) {
    Ok(content) => content,
    Err(_err) => {
      print!("cannot read config file: {}", path.to_str().unwrap());
      return;
    }
  };
  let config: Config = match serde_yaml::from_str(&String::from_utf8_lossy(&config_content)) {
    Ok(config) => config,
    Err(err) => {
      if err.to_string().starts_with("missing field") {
        Config::default()
      } else {
        println!("{}", err);
        return;
      }
    }
  };
  let cur_entry_id = config.entries.iter().map(|i| i.id).max().unwrap_or(0);
  let mut rtodo = Rtodo {
    conf_path: path.to_str().unwrap().to_string(),
    config,
    cur_entry_id,
  };
  match opt {
    Operation::Add(entry) => {
      rtodo.add_entry(entry);
    }
    Operation::StartDaemon() => match daemon::start_server(rtodo) {
      Ok(_) => {
        return;
      }
      Err(err) => {
        println!("{}", err);
        return;
      }
    },
    _other => (),
  }
  match rtodo.write_conf() {
    Ok(_) => (),
    Err(err) => {
      println!("{}", err)
    }
  }
}

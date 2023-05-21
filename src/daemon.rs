use std::sync::{RwLock, Arc};
use std::error::Error;
use crate::types::*;
use crate::server::start_server;
use std::thread;
use std::time;

pub fn start_daemon(rtodo: RwLock<Rtodo>) -> Result<(), Box<dyn Error>> {
    let rtodo = Arc::new(rtodo);
    let rtodo_move = rtodo.clone();
    thread::spawn(move || {
        start_server(&rtodo_move)
    });
    thread::spawn(move || {
        loop {
            thread::sleep(time::Duration::from_millis(100));
            
        }
    });
    Ok(())
}
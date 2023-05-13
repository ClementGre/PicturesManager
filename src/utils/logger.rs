use serde::{Serialize, Deserialize};

use super::utils::cmd_arg;

#[repr(usize)]
#[derive(Debug, Hash, Serialize, Deserialize)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Serialize, Deserialize)]
struct LoggingArgs<'a> {
    message: &'a str,
    level: Level
}
#[allow(dead_code)]
pub fn error(msg: &str){
    log(msg, Level::Error)
}
#[allow(dead_code)]
pub fn warn(msg: &str){
    log(msg, Level::Warn)
}
#[allow(dead_code)]
pub fn info(msg: &str){
    log(msg, Level::Info)
}
#[allow(dead_code)]
pub fn debug(msg: &str){
    log(msg, Level::Debug)
}
#[allow(dead_code)]
pub fn tr(msg: &str){
    log(msg, Level::Trace)
}

pub fn log(msg: &str, level: Level){
    let message = msg.to_owned();
    cmd_arg("log_from_front", &LoggingArgs{message: message.as_str(), level});
}
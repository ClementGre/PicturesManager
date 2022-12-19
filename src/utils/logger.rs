use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;

use crate::invoke;

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
    invoke("log_from_front", to_value(&LoggingArgs{message: &*msg, level}).unwrap());
}
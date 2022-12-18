use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[repr(usize)]
#[derive(Debug, Hash, Serialize, Deserialize)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct LoggingArgs<'a> {
    message: &'a str,
    level: Level
}

pub fn error(msg: &str){
    log(msg, Level::Error)
}
pub fn warn(msg: &str){
    log(msg, Level::Warn)
}
pub fn info(msg: &str){
    log(msg, Level::Info)
}
pub fn debug(msg: &str){
    log(msg, Level::Debug)
}
pub fn tr(msg: &str){
    log(msg, Level::Trace)
}

pub fn log(msg: &str, level: Level){
    invoke("log_from_front", to_value(&LoggingArgs{message: &*msg, level}).unwrap());
}
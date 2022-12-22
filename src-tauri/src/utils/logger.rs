use log::{debug, error, info, trace, warn, LevelFilter};
use serde::{Serialize, Deserialize};
use tauri::{plugin::TauriPlugin, Runtime};
use tauri_plugin_log::{LoggerBuilder, LogTarget, fern::colors::ColoredLevelConfig};

#[repr(usize)]
#[derive(Debug, Hash, Serialize, Deserialize)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[tauri::command]
pub fn log_from_front(message: &str, level: Level) {
    match level {
        Level::Error => error!("{}", message),
        Level::Warn => warn!("{}", message),
        Level::Info => info!("{}", message),
        Level::Debug => debug!("{}", message),
        Level::Trace => trace!("{}", message),
    }
}

pub fn get_logger_plugin<R: Runtime>() -> TauriPlugin<R>{

    let format =
    time::format_description::parse("[hour]:[minute]:[second]")
        .unwrap();

    let colors = ColoredLevelConfig::default();

    LoggerBuilder::new()
                .targets([LogTarget::Stdout, LogTarget::LogDir, LogTarget::Webview])
                .level(LevelFilter::Debug)
                .level_for("pictures_manager", LevelFilter::Trace)
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "[{}][{}][{}] {}",
                        time::OffsetDateTime::now_utc().format(&format).unwrap(),
                        record.target().replacen("pictures_manager", "", 1),
                        colors.color(record.level()),
                        message
                    ))
                    })
                .build()
}

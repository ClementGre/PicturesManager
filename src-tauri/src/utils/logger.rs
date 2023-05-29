use log::{debug, error, info, trace, warn, LevelFilter};
use tauri::{plugin::TauriPlugin, Runtime};
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, Builder, LogTarget};

#[tauri::command]
pub fn log_from_front(message: &str, level: usize) {
    match level {
        1 => error!("{}", message),
        2 => warn!("{}", message),
        // 3 => info!("{}", message),
        4 => debug!("{}", message),
        5 => trace!("{}", message),
        _ => info!("{}", message),
    }
}

pub fn get_logger_plugin<R: Runtime>() -> TauriPlugin<R> {
    let format = time::format_description::parse("[hour]:[minute]:[second]").unwrap();

    let colors = ColoredLevelConfig::default();

    Builder::new()
        .targets([LogTarget::Stdout, LogTarget::LogDir, LogTarget::Webview])
        .level(LevelFilter::Debug)
        .level_for("pictures_manager", LevelFilter::Trace)
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {} {}",
                time::OffsetDateTime::now_utc().format(&format).unwrap(),
                record.target().replacen("pictures_manager", "", 1),
                colors.color(record.level()),
                message
            ))
        })
        .build()
}

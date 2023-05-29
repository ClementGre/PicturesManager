use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};
use serde::{Deserialize, Serialize};

use super::utils::cmd_arg;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: LevelFilter = LevelFilter::Info;
#[cfg(debug_assertions)]
const LOG_LEVEL: LevelFilter = LevelFilter::Trace;

static LOGGER: BackendLogger = BackendLogger;

pub fn init_backend_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LOG_LEVEL))
}

#[derive(Serialize, Deserialize)]
struct LoggingArgs<'a> {
    message: &'a str,
    level: usize,
}

struct BackendLogger;

impl Log for BackendLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= LOG_LEVEL
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            cmd_arg(
                "log_from_front",
                &LoggingArgs {
                    message: record.args().to_string().as_str(),
                    level: record.level() as usize,
                },
            );
        }
    }

    fn flush(&self) {}
}

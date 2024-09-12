use std::io::Write;

use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

/// Initializes the logger for the application.
///
/// The logger is configured to output messages in the following format:
/// `YYYY-MM-DDTHH:MM:SS [LEVEL] - MESSAGE`
///
/// The default log level is `Info`.

pub fn init_logger() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}

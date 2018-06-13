extern crate fern;
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::fmt::Arguments;

use log::{Level, Record};

fn map_level_to_stackdriver_string(level: Level) -> &'static str {
    match level {
        Level::Error => "ERROR",
        Level::Warn => "WARNING",
        Level::Info => "INFO",
        Level::Debug => "DEBUG",
        Level::Trace => "DEBUG",
    }
}

impl<'a, 'b> From<(&'b Arguments<'a>, &'b Record<'a>)> for StackdriverLogLine {
    fn from((args, record): (&'b Arguments<'a>, &'b Record<'a>)) -> Self {
        StackdriverLogLine {
            severity: Some(map_level_to_stackdriver_string(record.level()).into()),
            message: format!("{}", args),
            file: record.file().map(String::from),
            line: record.line(),
            module_path: record.module_path().map(String::from),
            target: Some(record.target().into()),
        }
    }
}

#[derive(Serialize)]
pub struct StackdriverLogLine {
    severity: Option<String>,
    message: String,
    file: Option<String>,
    line: Option<u32>,
    module_path: Option<String>,
    target: Option<String>,
}

// Stolen from https://github.com/FatWhaleCorp/bookkeeper/blob/ee40830ef8fc79ca1c458811de55a74b5ab00a35/src/main.rs#L25-L40
pub fn init_default_logger() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            let stackdriver_log_line = StackdriverLogLine::from((message, record));
            out.finish(format_args!(
                "{}",
                serde_json::to_string(&stackdriver_log_line).unwrap()
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("hyper", log::LevelFilter::Info)
        .level_for("mio", log::LevelFilter::Info)
        .level_for("tokio_core", log::LevelFilter::Info)
        .level_for("tokio_reactor", log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}

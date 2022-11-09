//! Global logger

use log::{self, Level, LevelFilter, Log, Metadata, Record};

use crate::console;

/// a simple logger
struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let (level_str, color) = match record.level() {
            Level::Error => ("[E] ", LogColor::Red),
            Level::Warn => ("[W] ", LogColor::Yellow),
            Level::Info => ("[I] ", LogColor::Cyan),
            Level::Debug => ("[D] ", LogColor::Green),
            Level::Trace => ("[T] ", LogColor::Purple),
        };
        let file_loc = record.file().unwrap_or("??");
        let line_loc = record.line().unwrap_or(0);
        // Just dump to sbi console
        console::print(format_args!(
            "{}[U]{}{}:{}: {}{}",
            color.as_terminal_string(),
            level_str,
            file_loc,
            line_loc,
            record.args(),
            LogColor::Reset.as_terminal_string(),
        ));
    }
    fn flush(&self) {}
}

/// ANSI style codes for basic colors.
#[allow(dead_code)]
enum LogColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
    Reset,
}
impl LogColor {
    fn as_terminal_string(&self) -> &'static str {
        match *self {
            // \x1b is the ESC character (0x1B)
            LogColor::Black => "\x1b[30m",
            LogColor::Red => "\x1b[31m",
            LogColor::Green => "\x1b[32m",
            LogColor::Yellow => "\x1b[33m",
            LogColor::Blue => "\x1b[34m",
            LogColor::Purple => "\x1b[35m",
            LogColor::Cyan => "\x1b[36m",
            LogColor::White => "\x1b[37m",
            LogColor::Reset => "\x1b[0m\n",
        }
    }
}

/// initiate logger
pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => LevelFilter::Error,
        Some("WARN") => LevelFilter::Warn,
        Some("INFO") => LevelFilter::Info,
        Some("DEBUG") => LevelFilter::Debug,
        Some("TRACE") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
}

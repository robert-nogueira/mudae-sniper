use crate::{settings::SETTINGS, utils::get_local_time};
use chrono::Timelike;
use log::kv::{Key, Value, VisitSource};
use log::{Level, SetLoggerError};

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= SETTINGS.log_level
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata())
            || record.target() != "mudae_sniper"
        {
            return;
        }

        let color = match record.level() {
            Level::Trace => "\x1b[38;2;203;166;247m", // blue
            Level::Debug => "\x1b[38;2;166;227;161m", // green
            Level::Info => "\x1b[38;2;89;180;250m",   // mauve
            Level::Warn => "\x1b[38;2;249;226;175m",  // yellow
            Level::Error => "\x1b[38;2;243;139;168m", // red
        };
        let end_color = "\x1b[0m";
        let now = get_local_time().with_nanosecond(0).unwrap();
        let level = record.level();
        let args = record.args();
        print!("{color}[\x1b[1m{level}] [{now}]: {end_color}{args}",);

        struct KVPrinter<'a> {
            color: &'a str,
            end_color: &'a str,
        }
        impl<'kvs, 'a> VisitSource<'kvs> for KVPrinter<'a> {
            fn visit_pair(
                &mut self,
                key: Key<'kvs>,
                value: Value<'kvs>,
            ) -> Result<(), log::kv::Error> {
                print!(
                    " \x1b[3m{}{}={}{}\x1b[0m",
                    self.color, key, value, self.end_color
                );
                Ok(())
            }
        }

        let mut printer = KVPrinter { color, end_color };
        let _ = record.key_values().visit(&mut printer);
        println!();
    }

    fn flush(&self) {}
}

static LOGGER: Logger = Logger;

pub fn init_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(SETTINGS.log_level))
}

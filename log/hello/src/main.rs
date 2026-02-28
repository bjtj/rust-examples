use log::{error, warn, info, debug, trace};
use log::{Record, Metadata, LevelFilter};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

fn main() {

    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Debug);
    
    println!("++main");
    error!("Hello, world! (error)");
    warn!("Hello, world! (warn)");
    info!("Hello, world! (info)");
    debug!("Hello, world! (debug)");
    trace!("Hello, world! (trace)");
    println!("--main");
}

#[macro_use]
extern crate log;
extern crate log4rs;
extern crate nonblock;
extern crate termion;
extern crate tokio;
extern crate tokio_threadpool;
extern crate tui;

mod app;
mod runtime;
mod errors;
mod ui;

use app::CosmosApp;

use std::env;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

fn main() {
    // initialize logger
    let output = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} {h({l})} {t} - {m}{n}",
        )))
        .build("log/output.log")
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("output", Box::new(output)))
        .logger(
            Logger::builder()
                .appender("output")
                .additive(false)
                .build("app::output", LevelFilter::Debug),
        )
        .build(Root::builder().appender("output").build(LevelFilter::Debug))
        .unwrap();

    let log_handle = log4rs::init_config(config).unwrap();
    // start cosmos
    CosmosApp::init(env::args()).start_application();
}

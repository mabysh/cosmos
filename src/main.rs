#[macro_use]
extern crate log;
extern crate env_logger;
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

fn main() {
    // initialize logger
    env_logger::init();
    // start cosmos
    CosmosApp::init(env::args()).start_application();
}

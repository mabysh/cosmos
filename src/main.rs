extern crate nonblock;
extern crate termion;
extern crate tokio;
extern crate tui;

mod app;
mod errors;

use app::CosmosApp;

use std::env;

fn main() {
    CosmosApp::configure()
        .from_args(env::args())
        .build()
        .start_application();
}

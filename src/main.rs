extern crate nonblock;
extern crate termion;
extern crate tokio;
extern crate tui;

mod app;

use app::CosmosApp;

fn main() {
    let app = CosmosApp::initialize();
}

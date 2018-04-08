pub mod runtime;
pub mod ui;

use std::io;
use tokio::runtime::Runtime as TokioRuntime;
use tui::backend::RawBackend;
use tui::Terminal;
use self::runtime::Runtime;

pub struct CosmosApp {
    runtime: Runtime,
    terminal: Terminal<RawBackend>,
}

impl CosmosApp {
    fn initialize() -> CosmosApp {
        let ctx = runtime::init_runtime(None);
        let term = init_terminal().expect("Failed to initialize terminal");
        CosmosApp {
            runtime: ctx,
            terminal: term,
        }
    }

    pub fn get_runtime_mut(&mut self) -> &mut Runtime {
        &mut self.runtime
    }

    pub fn get_term_mut(&mut self) -> &mut Terminal<RawBackend> {
        &mut self.terminal
    }
}

pub fn start_cosmos() {
    //// SPIN UP EVENT LOOP IN RIGHT WAY
    let app = CosmosApp::initialize();
    runtime::listen_stdin(app.get_runtime_mut());
}

fn init_terminal() -> Result<Terminal<RawBackend>, io::Error> {
    let backend = RawBackend::new()?;
    Terminal::new(backend)
}

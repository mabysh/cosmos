pub mod runtime;
pub mod ui;

use tokio::runtime::Runtime as TokioRuntime;
use tui::backend::RawBackend;
use tui::Terminal;

use self::runtime::Runtime;

use std::env::Args;
use std::io::{self, ErrorKind};
use std::error;

pub struct CosmosApp {
    runtime: Runtime,
    config: CosmosConfig,
    terminal: Terminal<RawBackend>,
}

impl CosmosApp {
    pub fn configure() -> CosmosConfig {
        CosmosConfig::default()
    }

    pub fn start_application() {}

    pub fn get_runtime_mut(&mut self) -> &mut Runtime {
        &mut self.runtime
    }

    pub fn get_term_mut(&mut self) -> &mut Terminal<RawBackend> {
        &mut self.terminal
    }
}

struct CosmosConfig {
    thrd_pool_size: usize,
}

impl CosmosConfig {
    fn default() -> CosmosConfig {
        CosmosConfig { thrd_pool_size: 4 }
    }

    pub fn from_args(args: Args) -> CosmosConfig {
        let config = CosmosConfig::default();
        let config_valid = true;
        loop {
            match args.next() {
                Some(arg) => match parse_arg(&args, &arg, config) {
                    Ok(config) => {
                        config = config;
                    }
                    Err(e) => {
                        config_valid = false;
                    }
                },
                None => {
                    break;
                }
            }
        }
        if config_valid {
            config
        } else {
            CosmosConfig::default()
        }
    }

    pub fn with_pool_size(self, size: usize) -> CosmosConfig {
        // TODO CHECK AND LOG INVALID PARAMETER
        self.thrd_pool_size = size;
        self
    }

    fn build(self) -> CosmosApp {}

    pub fn start_application(self) -> CosmosApp {
        let rt = Runtime::with_pool_size(self.thrd_pool_size);
        let term = init_terminal().expect("Failed to initialize terminal");
        CosmosApp {
            runtime: rt,
            terminal: term,
            config: self,
        }
    }
}

fn parse_arg(
    args: &Args,
    arg: &str,
    config: CosmosConfig,
) -> Result<CosmosConfig, Box<error::Error>> {
    match arg {
        "-p" => match args.next() {
            Some(next_arg) => match next_arg.parse::<usize>() {
                Err(e) => return Err(Box::new(e)),
                Ok(size) => return Ok(config.with_pool_size(size)),
            },
            None => {
                let e = io::Error::from(ErrorKind::NotFound);
                return Err(Box::new(e));
            }
        },
        _ => {
            // log invalid parameter
            return Err(Box::new(io::Error::from(ErrorKind::InvalidInput)));
        }
    }
}

pub fn start_cosmos(args: Option<Args>) {
    match args {
        Some(a) => {}
        None => {}
    }
    let app = CosmosApp::initialize();
    runtime::listen_stdin(app.get_runtime_mut());
}

fn init_terminal() -> Result<Terminal<RawBackend>, io::Error> {
    let backend = RawBackend::new()?;
    Terminal::new(backend)
}

pub mod runtime;
pub mod ui;

use tui::backend::RawBackend;
use tui::Terminal;

use self::runtime::Runtime;
use super::errors::{CosmError, CosmErrorKind};

use std::env::Args;
use std::io;

pub struct CosmosApp {
    runtime: Runtime,
    config: CosmosConfig,
    terminal: Terminal<RawBackend>,
}

impl CosmosApp {
    pub fn configure() -> CosmosConfig {
        CosmosConfig::default()
    }

    pub fn start_application(&mut self) {
        runtime::process_user_input(self);
    }

    pub fn get_runtime_mut(&mut self) -> &mut Runtime {
        &mut self.runtime
    }

    pub fn get_term_mut(&mut self) -> &mut Terminal<RawBackend> {
        &mut self.terminal
    }
}

pub struct CosmosConfig {
    thrd_pool_size: usize,
}

impl CosmosConfig {
    fn default() -> CosmosConfig {
        CosmosConfig { thrd_pool_size: 4 }
    }

    pub fn from_args(mut self, mut args: Args) -> CosmosConfig {
        let mut args_valid = true;
        loop {
            match args.next() {
                Some(arg) => match parse_arg(&mut args, &arg, &mut self) {
                    Ok(_) => {}
                    Err(_e) => {
                        //TODO log errors
                        args_valid = false;
                        break;
                    }
                },
                None => {
                    break;
                }
            }
        }
        if args_valid {
            self
        } else {
            CosmosConfig::default()
        }
    }

    pub fn with_thread_pool_size(mut self, size: usize) -> CosmosConfig {
        // TODO CHECK AND LOG INVALID PARAMETER
        self.thrd_pool_size = size;
        self
    }

    fn set_thread_pool_size(&mut self, size: usize) {
        // TODO CHECK AND LOG INVALID PARAMETER
        self.thrd_pool_size = size;
    }

    pub fn build(self) -> CosmosApp {
        let rt = Runtime::with_pool_size(self.thrd_pool_size);
        let term = init_terminal().expect("Failed to initialize terminal");
        CosmosApp {
            runtime: rt,
            terminal: term,
            config: self,
        }
    }
}

fn parse_arg(args: &mut Args, arg: &str, config: &mut CosmosConfig) -> Result<(), CosmError> {
    match arg {
        "-p" => match args.next() {
            Some(next_arg) => match next_arg.parse::<usize>() {
                Err(e) => {
                    return Err(CosmError::new(
                        format!("Invalid pool size: {}", next_arg).as_str(),
                        CosmErrorKind::InvalidArg,
                        Some(CosmError::from_std_error(&e)),
                    ))
                }
                Ok(size) => {
                    config.set_thread_pool_size(size);
                    return Ok(());
                }
            },
            None => {
                return Err(CosmError::new(
                    "Pool size parameter not found.",
                    CosmErrorKind::MissingArg,
                    None,
                ));
            }
        },
        _ => {
            // log invalid parameter
            return Err(CosmError::new(
                format!("Unexpected argument: '{}'", arg).as_str(),
                CosmErrorKind::InvalidArg,
                None,
            ));
        }
    }
}

fn init_terminal() -> Result<Terminal<RawBackend>, io::Error> {
    let backend = RawBackend::new()?;
    Terminal::new(backend)
}

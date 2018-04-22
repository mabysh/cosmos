use tui::backend::RawBackend;
use tui::Terminal;

use errors::{CosmError, CosmErrorKind};
use runtime;
use ui;

use std::env::Args;

pub struct CosmosApp {
    config: CosmosConfig,
    terminal: Terminal<RawBackend>,
}

impl CosmosApp {
    pub fn init(args: Args) -> CosmosApp {
        debug!("Initializing cosmos..");
        let terminal = ui::init_terminal().expect("Failed to initialize terminal");
        let config = CosmosConfig::from_args(args);
        CosmosApp {
            config: config,
            terminal: terminal,
        }
    }

    pub fn start_application(&mut self) {
        debug!("Starting application..");
        runtime::start_application(self);
    }

    pub fn term_mut(&mut self) -> &mut Terminal<RawBackend> {
        &mut self.terminal
    }

    pub fn config_mut(&mut self) -> &mut CosmosConfig {
        &mut self.config
    }
}

pub struct CosmosConfig {
    thrd_pool_size: usize,
}

impl CosmosConfig {
    fn default() -> Self {
        CosmosConfig { thrd_pool_size: 4 }
    }

    pub fn from_args(mut args: Args) -> Self {
        let mut config = CosmosConfig::default();
        // skip first arg
        args.next();
        loop {
            match args.next() {
                Some(arg) => match parse_arg(&mut args, &arg, &mut config) {
                    Ok(_) => {}
                    Err(e) => {
                        // these are not fatal errors. if some args failed to parse, default values
                        // will be used
                        let er = CosmError::new(
                            "Args parsing error",
                            CosmErrorKind::ParseArgsError,
                            Some(e),
                        );
                        error!("{}", er);
                    }
                },
                None => {
                    break;
                }
            }
        }
        config
    }

    pub fn with_thread_pool_size(&mut self, size: usize) -> &mut Self {
        // TODO CHECK AND LOG INVALID PARAMETER
        self.thrd_pool_size = size;
        self
    }

    pub fn pool_size(&self) -> usize {
        self.thrd_pool_size
    }
}

fn parse_arg(args: &mut Args, arg: &str, config: &mut CosmosConfig) -> Result<(), CosmError> {
    match arg {
        "-p" => match args.next() {
            Some(next_arg) => match next_arg.parse::<usize>() {
                Err(e) => {
                    return Err(CosmError::new(
                        format!(
                            "Invalid pool size: {}. Using default value: {}.",
                            next_arg,
                            config.pool_size()
                        ).as_str(),
                        CosmErrorKind::InvalidArg,
                        Some(CosmError::from_std_error(&e)),
                    ))
                }
                Ok(size) => {
                    config.with_thread_pool_size(size);
                    return Ok(());
                }
            },
            None => {
                return Err(CosmError::new(
                    format!(
                        "Pool size parameter not found. Using default value: {}.",
                        config.pool_size()
                    ).as_str(),
                    CosmErrorKind::MissingArg,
                    None,
                ));
            }
        },
        _ => {
            return Err(CosmError::new(
                format!("Unexpected argument: '{}'", arg).as_str(),
                CosmErrorKind::InvalidArg,
                None,
            ));
        }
    }
}

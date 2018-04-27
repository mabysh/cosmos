pub mod events;

use std::io;

use tui::backend::RawBackend;
use tui::Terminal;

use nonblock::input_stream;

use tokio::prelude::*;
use tokio::prelude::future::ok;
use tokio::runtime::{Builder, Runtime};
use tokio::executor::thread_pool;

use tokio_threadpool::blocking;

use super::app::{CosmosApp, CosmosConfig};
use errors::CosmError;

pub fn start_application(app: &mut CosmosApp) {
    // setup tokio runtime
    let mut rt = init_tokio_runtime(app.config_mut());
    let executor = rt.executor();
    // create user input stream
    let user_input = input_stream(io::stdin(), Some(1024))
        .map_err(|e| {
            let er = CosmError::from_std_error(&e);
            error!("User input stream error: {}", er);
        })
        .for_each(move |input| {
            //
            //
            // All action will be here
            //
            //
            info!("{:?}", input);
            Ok(())
        });
    // spin it up
    debug!("Spawning stream..");
    rt.spawn(user_input);
    rt.shutdown_on_idle().wait().unwrap();
}

fn init_tokio_runtime(conf: &CosmosConfig) -> Runtime {
    debug!("Initializing tokio runtime..");
    let mut thrd_pool = thread_pool::Builder::new();
    thrd_pool.pool_size(conf.pool_size());
    let rt = Builder::new()
        .threadpool_builder(thrd_pool)
        .build()
        .expect("Failed to set up runtime");
    rt
}

pub fn init_terminal() -> Result<Terminal<RawBackend>, io::Error> {
    let backend = RawBackend::new()?;
    Terminal::new(backend)
}

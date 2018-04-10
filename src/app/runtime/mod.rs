pub mod events;

use nonblock::input_stream;

use std::io;
use std::env::Args;

use tokio::prelude::*;
use tokio::runtime::{Builder, Runtime as TokioRuntime};
use tokio::executor::thread_pool;

/// this struct represent application runtime context
pub struct Runtime {
    tokio_runtime: TokioRuntime,
}

impl Runtime {
    pub fn with_pool_size(size: usize) -> Runtime {
        let rt = setup_tokio_runtime(size);
        Runtime { tokio_runtime: rt }
    }

    pub fn spawn<F>(&self, f: F)
    where
        F: Future<Item = (), Error = ()> + 'static + Send,
    {
        &mut self.tokio_runtime.spawn(f);
    }
}

pub fn listen_stdin(rt: &mut Runtime) {
    let input = input_stream(io::stdin(), None)
        .for_each(|val| {
            println!("{:?}", val);
            Ok(())
        })
        .map_err(|e| eprintln!("{}", e));
    rt.spawn(input);
}

fn setup_tokio_runtime(pool_size: usize) -> TokioRuntime {
    let mut thrd_pool = thread_pool::Builder::new();
    thrd_pool.pool_size(pool_size);
    let rt = Builder::new()
        .threadpool_builder(thrd_pool)
        .build()
        .expect("Failed to set up runtime");
    rt
}

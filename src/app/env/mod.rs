use nonblock::input_stream;
use std::io;

use tokio::prelude::*;
use tokio::runtime::{Builder, Runtime};
use tokio::executor::thread_pool;

pub fn listen_stdin(rt: &mut Runtime) {
    let input = input_stream(io::stdin(), None)
        .for_each(|val| {
            println!("{:?}", val);
            Ok(())
        })
        .map_err(|e| eprintln!("{}", e));
    rt.spawn(input);
}

pub fn setup_thread_pool() -> Runtime {
    let mut thrd_pool = thread_pool::Builder::new();
    thrd_pool.pool_size(4);
    let rt = Builder::new()
        .threadpool_builder(thrd_pool)
        .build()
        .expect("Failed to set up runtime");
    rt
}

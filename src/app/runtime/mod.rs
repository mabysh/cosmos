pub mod events;

use nonblock::input_stream;

use std::io;

use tokio::prelude::*;
use tokio::runtime::{Builder, Runtime as TokioRuntime};
use tokio::executor::thread_pool;

use super::CosmosApp;
use super::CosmError;

/// this struct represent application runtime context
pub struct Runtime {
    tokio_runtime: TokioRuntime,
}

impl Runtime {
    pub fn with_pool_size(size: usize) -> Runtime {
        let rt = setup_tokio_runtime(size);
        Runtime { tokio_runtime: rt }
    }

    pub fn spawn<F>(&mut self, f: F)
    where
        F: Future<Item = (), Error = ()> + 'static + Send,
    {
        &mut self.tokio_runtime.spawn(f);
    }
}

pub fn process_user_input(app: &mut CosmosApp) {
    let user_input = input_stream(io::stdin(), None)
        .for_each(|val| {
            println!("{:?}", val);
            Ok(())
        })
        .map_err(|e| eprintln!("{}", e));
    app.get_runtime_mut().spawn(user_input);
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

pub enum JobStatus {
    Created,
    InProgress,
    Done,
}

pub struct CosmJob<F> {
    status: JobStatus,
    job: F,
}

impl<F, T> CosmJob<F>
where
    F: FnMut() -> T,
{
    pub fn new(job: F) -> CosmJob<F> {
        CosmJob {
            status: JobStatus::InProgress,
            job: job,
        }
    }

    fn run(&mut self) -> T {
        (self.job)()
    }
}

impl<F, T> Future for CosmJob<F>
where
    F: FnMut() -> T,
{
    type Item = T;
    type Error = CosmError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.status {
            JobStatus::Created => {
                // run job, return value
                self.status = JobStatus::InProgress;
                let output = (self.job)();
            }
        }
        let output = self.run();
        Ok(Async::NotReady)
    }
}

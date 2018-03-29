pub mod env;
pub mod events;

use tokio::runtime::Runtime;

pub struct CosmosApp {
    thread_pool: Runtime,
}

impl CosmosApp {
    fn initialize() -> CosmosApp {
        let pool = env::setup_thread_pool();
        CosmosApp { thread_pool: pool }
    }

    fn get_pool_mut(&mut self) -> &mut Runtime {
        &mut self.thread_pool
    }
}

pub fn start_cosmos() -> CosmosApp {
    let mut app = CosmosApp::initialize();
    env::listen_stdin(app.get_pool_mut());
    app
}

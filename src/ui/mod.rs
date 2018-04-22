use std::io;
use tui::backend::RawBackend;
use tui::Terminal;

pub fn init_terminal() -> Result<Terminal<RawBackend>, io::Error> {
    let backend = RawBackend::new()?;
    Terminal::new(backend)
}

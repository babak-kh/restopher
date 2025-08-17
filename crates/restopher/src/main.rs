mod app;
mod collection;
mod components;
mod env_replacer;
mod environments;
mod graphql;
mod keys;
mod layout;
mod logger;
mod main_windows;
mod models;
mod request;
mod styles;

use request::Request;
use std::io;
use tokio;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

pub(crate) const START_ENV_TOKEN: &str = "{{";
pub(crate) const END_ENV_TOKEN: &str = "}}";
pub(crate) const ENV_PATH: &str = "environments";
pub(crate) const COLLECTION_PATH: &str = "collections";
pub(crate) const DATA_DIRECTORY: &str = "/home/babak/.config/restopher";

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    //    setup_terminal()?;
    //    env::set_var("RUST_BACKTRACE", "1");
    logger::initialize_logging().unwrap();
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let app = app::App::new();
    app.run(&mut terminal).await;
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

mod app;
mod collection;
mod components;
mod environments;
mod keys;
mod layout;
mod logger;
mod main_windows;
mod request;
mod styles;

use request::Request;
use std::{io};
use tokio;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

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
    let res = app.run(&mut terminal).await;
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

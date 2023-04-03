use std::{io, ops::Div, slice::Chunks, sync::mpsc, thread};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    terminal,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

#[derive(Debug)]
enum Windows {
    Address,
    Response,
}

struct App {
    selected_window: Windows,
    current_url: String,
}
impl App {
    fn new() -> Self {
        App {
            selected_window: Windows::Address,
            current_url: String::new(),
        }
    }
    fn up(&mut self) {
        match self.selected_window {
            Windows::Address => self.selected_window = Windows::Response,
            Windows::Response => self.selected_window = Windows::Address,
        };
    }
    fn down(&mut self) {
        match self.selected_window {
            Windows::Address => self.selected_window = Windows::Response,
            Windows::Response => self.selected_window = Windows::Address,
        };
    }
}

fn main() -> Result<(), io::Error> {
    //    setup_terminal()?;
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let res = run_app(&mut terminal);
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }
    Ok(())
}

fn run_app<B: Backend>(term: &mut Terminal<B>) -> Result<(), std::io::Error> {
    let mut app = App::new();
    loop {
        term.draw(|f| ui(f, &app))?;
        if let Event::Key(key) = event::read()? {
            match app.selected_window {
                Windows::Address => {
                    match key.modifiers {
                        event::KeyModifiers::CONTROL => match key.code {
                            event::KeyCode::Char('q') => break,
                            event::KeyCode::Char('j') => app.down(),
                            event::KeyCode::Char('k') => app.up(),
                            event::KeyCode::Enter => app.call()
                            _ => (),
                        },
                        _ => (),
                    }
                    match key.code {
                        event::KeyCode::Char(x) => app.current_url.push(x),
                        _ => (),
                    }
                }
                Windows::Response => match key.modifiers {
                    event::KeyModifiers::CONTROL => match key.code {
                        event::KeyCode::Char('q') => break,
                        event::KeyCode::Char('j') => app.down(),
                        event::KeyCode::Char('k') => app.up(),
                        _ => (),
                    },
                    _ => (),
                },
            }
        }
    }
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(f.size());
    let address = Block::default()
        .title("Address")
        .style(Style::default().bg(Color::Black))
        .borders(Borders::ALL);
    let body = Block::default()
        .title("Body")
        .style(Style::default().bg(Color::Black))
        .borders(Borders::ALL);
    let data = Paragraph::new(app.current_url.clone()).wrap(tui::widgets::Wrap { trim: true });

    match app.selected_window {
        Windows::Address => {
            let address = address.border_type(tui::widgets::BorderType::Thick);
            let data = data.block(address);
            f.render_widget(data, chunks[0]);
            f.render_widget(body, chunks[1])
        }
        Windows::Response => {
            let body = body.border_type(tui::widgets::BorderType::Thick);
            let data = data.block(address);
            f.render_widget(data, chunks[0]);
            f.render_widget(body, chunks[1])
        }
    }
}

use std::{io, ops::Div, slice::Chunks, sync::mpsc, thread};
mod layout;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

#[derive(Debug)]
enum Windows {
    Address,
    Response,
    Verb,
}
#[derive(Debug)]
enum HttpVerb {
    GET,
    POST,
    PUT,
    DELETE,
}
impl HttpVerb {
    fn to_string(&self) -> String {
        match self {
            HttpVerb::GET => "GET".to_string(),
            HttpVerb::POST => "POST".to_string(),
            HttpVerb::DELETE => "DELETE".to_string(),
            HttpVerb::PUT => "PUT".to_string(),
        }
    }
    fn down(&self) -> Self {
        match self {
            HttpVerb::GET => HttpVerb::POST,
            HttpVerb::POST => HttpVerb::PUT,
            HttpVerb::PUT => HttpVerb::DELETE,
            HttpVerb::DELETE => HttpVerb::GET,
        }
    }
    fn up(&self) -> Self {
        match self {
            HttpVerb::GET => HttpVerb::DELETE,
            HttpVerb::POST => HttpVerb::GET,
            HttpVerb::PUT => HttpVerb::POST,
            HttpVerb::DELETE => HttpVerb::PUT,
        }
    }
}

struct App {
    selected_window: Windows,
    current_url: String,
    response: String,
    verb: HttpVerb,
}
impl App {
    fn new() -> Self {
        App {
            selected_window: Windows::Address,
            current_url: String::new(),
            response: String::new(),
            verb: HttpVerb::GET,
        }
    }
    fn up(&mut self) {
        match self.selected_window {
            Windows::Address => self.selected_window = Windows::Response,
            Windows::Response => self.selected_window = Windows::Address,
            Windows::Verb => (),
        };
    }
    fn down(&mut self) {
        match self.selected_window {
            Windows::Address => self.selected_window = Windows::Response,
            Windows::Response => self.selected_window = Windows::Address,
            Windows::Verb => self.selected_window = Windows::Response,
        };
    }
    fn right(&mut self) {
        match self.selected_window {
            Windows::Address => (),
            Windows::Response => (),
            Windows::Verb => self.selected_window = Windows::Address,
        };
    }
    fn left(&mut self) {
        match self.selected_window {
            Windows::Address => self.selected_window = Windows::Verb,
            Windows::Response => (),
            Windows::Verb => (),
        };
    }
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    //    setup_terminal()?;
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let res = run_app(&mut terminal).await;
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

async fn call(url: String) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

async fn run_app<B: Backend>(term: &mut Terminal<B>) -> Result<(), std::io::Error> {
    let mut app = App::new();
    loop {
        term.draw(|f| ui(f, &app))?;
        if let Event::Key(key) = event::read()? {
            match key.modifiers {
                event::KeyModifiers::CONTROL => match key.code {
                    event::KeyCode::Char('q') => break,
                    event::KeyCode::Char('j') => {
                        app.down();
                        continue;
                    }
                    event::KeyCode::Char('k') => {
                        app.up();
                        continue;
                    }
                    event::KeyCode::Char('h') => {
                        app.left();
                        continue;
                    }
                    event::KeyCode::Char('l') => {
                        app.right();
                        continue;
                    }
                    _ => (),
                },
                _ => (),
            }
            match app.selected_window {
                Windows::Address => {
                    match key.modifiers {
                        event::KeyModifiers::CONTROL => match key.code {
                            event::KeyCode::Char('a') => {
                                app.response = call(app.current_url.clone()).await.unwrap();
                                continue;
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                    match key.code {
                        event::KeyCode::Char(x) => app.current_url.push(x),
                        event::KeyCode::Backspace => {
                            app.current_url.pop();
                        }
                        _ => (),
                    }
                }
                Windows::Response => (),
                Windows::Verb => match key.code {
                    event::KeyCode::Up => app.verb = app.verb.up(),
                    event::KeyCode::Down => app.verb = app.verb.down(),
                    _ => (),
                },
            }
        }
    }
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let address = Block::default()
        .title("Address")
        .style(Style::default().bg(Color::Black))
        .borders(Borders::ALL);
    let verb = Block::default()
        .title("Verb")
        .style(Style::default().bg(Color::Black))
        .borders(Borders::ALL);
    let body = Block::default()
        .title("Body")
        .style(Style::default().bg(Color::Black))
        .borders(Borders::ALL);
    let l = layout::LayoutBuilder::default(f);
    let data = Paragraph::new(app.current_url.clone()).wrap(tui::widgets::Wrap { trim: true });
    let resp = Paragraph::new(app.response.clone()).wrap(tui::widgets::Wrap { trim: true });
    let verb_str = Paragraph::new(app.verb.to_string()).wrap(tui::widgets::Wrap { trim: true });
    match app.selected_window {
        Windows::Address => {
            let address = address.border_type(tui::widgets::BorderType::Thick);
            let data = data.block(address);
            let resp = resp.block(body);
            let verb = verb_str.block(verb);
            f.render_widget(verb, l.verb);
            f.render_widget(data, l.address);
            f.render_widget(resp, l.body);
        }
        Windows::Response => {
            let body = body.border_type(tui::widgets::BorderType::Thick);
            let data = data.block(address);
            let resp = resp.block(body);
            let verb = verb_str.block(verb);
            f.render_widget(verb, l.verb);
            f.render_widget(data, l.address);
            f.render_widget(resp, l.body);
        }
        Windows::Verb => {
            let verb = verb.border_type(tui::widgets::BorderType::Thick);
            let data = data.block(address);
            let resp = resp.block(body);
            let verb = verb_str.block(verb);
            f.render_widget(verb, l.verb);
            f.render_widget(data, l.address);
            f.render_widget(resp, l.body);
        }
    }
}

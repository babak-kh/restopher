mod app;
mod layout;
mod request;
mod response;
use std::io;
use tokio;

use app::{App, Windows};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Tabs, ListState, TableState},
    Frame, Terminal, layout::Constraint,
};

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

async fn run_app<B: Backend>(term: &mut Terminal<B>) -> Result<(), std::io::Error> {
    let mut app = app::App::new();
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
                    event::KeyCode::Char('t') => {
                        app.change_request_tab();
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
                                app.call_request().await.unwrap();
                                continue;
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                    match key.code {
                        event::KeyCode::Char(x) => app.add_address(x),
                        event::KeyCode::Backspace => {
                            app.pop_address();
                        }
                        _ => (),
                    }
                }
                Windows::Response => (),
                Windows::Verb => match key.code {
                    event::KeyCode::Up => app.verb_up(),
                    event::KeyCode::Down => app.verb_down(),
                    _ => (),
                },
                Windows::RequestData => match key.modifiers {
                    event::KeyModifiers::CONTROL => match key.code {
                        event::KeyCode::Char('n') => app.add_header("".to_string(), "".to_string()),
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
    let request_data = Block::default()
        .title("Request Data")
        .style(Style::default().bg(Color::Black))
        .borders(Borders::ALL);
    let titles: Vec<Spans> = app
        .req_tabs
        .req_tabs
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at();
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();
    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Request data tabs"),
        )
        .select(app.req_tabs.selected_idx)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );

    let l = layout::LayoutBuilder::default(f);
    // Headers Table
    match app.req_tabs.selected {
        app::RequestTabs::Headers(_, _) => {
            let selected_style = Style::default().add_modifier(Modifier::REVERSED);
            let normal_style = Style::default().bg(Color::Blue);
            if let Some(headers) = app.headers() {
                let headers = headers.clone();
                let rows = headers.iter().map(|item| {
                    let height = headers.len() + 1;
                    let cells = { vec![Cell::from(item.0.clone()), Cell::from(item.1.clone())] };
                    Row::new(cells).height(height as u16).bottom_margin(1)
                });
                let t = Table::new(rows)
                    .block(request_data)
                    .highlight_style(selected_style)
                    .highlight_symbol(">> ")
                    .widths(&[
                        Constraint::Percentage(50),
                        Constraint::Length(30),
                        Constraint::Min(10),
                    ]);
                f.render_stateful_widget(t, l.req_data, &mut TableState::default());
            }
        }
        app::RequestTabs::Params(_, _) => todo!(),
        app::RequestTabs::Body(_, _) => todo!(),
    }
    let data = Paragraph::new(app.address().clone().unwrap_or("".to_string()))
        .wrap(tui::widgets::Wrap { trim: true });
    let resp = Paragraph::new(app.response_body().clone()).wrap(tui::widgets::Wrap { trim: true });
    let verb_str = Paragraph::new(app.verb()).wrap(tui::widgets::Wrap { trim: true });

    f.render_widget(tabs, l.req_tabs);
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
        Windows::RequestData => {
            let data = data.block(address);
            let resp = resp.block(body);
            let verb = verb_str.block(verb);
            f.render_widget(verb, l.verb);
            f.render_widget(data, l.address);
            f.render_widget(resp, l.body);
        }
    }
}

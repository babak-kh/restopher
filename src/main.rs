mod app;
mod layout;
mod request;
mod response;
mod environments;
use std::io;
use tokio;

use app::{App, Windows};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Tabs},
    Frame, Terminal,
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
                        app.temp_header_param_idx = 0;
                        continue;
                    }
                    event::KeyCode::Char('b') => {
                        app.change_response_tab();
                        continue;
                    }
                    event::KeyCode::Char('e') => {
                        app.show_environments = true;
                        continue;
                    }
                    event::KeyCode::Char('a') => {
                        app.call_request()
                            .await
                            .or_else(|e| {
                                app.error_pop_up = (true, Some(e));
                                Ok::<String, app::Error>("done".to_string())
                            })
                            .unwrap();
                        continue;
                    }
                    _ => (),
                },
                _ => (),
            }
            match key.code {
                event::KeyCode::Esc => {
                    if app.error_pop_up.0 {
                        app.error_pop_up = (false, None);
                    }
                }
                _ => (),
            }
            match app.selected_window {
                Windows::Address => match key.code {
                    event::KeyCode::Char(x) => app.add_address(x),
                    event::KeyCode::Backspace => {
                        app.pop_address();
                    }
                    _ => (),
                },

                Windows::Response => (),
                Windows::Verb => match key.code {
                    event::KeyCode::Up => app.verb_up(),
                    event::KeyCode::Down => app.verb_down(),
                    _ => (),
                },

                Windows::RequestData => match app.req_tabs.selected {
                    app::RequestTabs::Headers(_, _) => {
                        if app.has_new_header() {
                            match key.code {
                                event::KeyCode::Char(x) => app.add_to_kv(x),
                                event::KeyCode::Backspace => {
                                    app.remove_from_kv();
                                }
                                event::KeyCode::Tab => app.change_active(),
                                _ => (),
                            };
                        };
                        match key.modifiers {
                            event::KeyModifiers::CONTROL => match key.code {
                                event::KeyCode::Char('n') => app.initiate_new_header(),
                                event::KeyCode::Char('d') | event::KeyCode::Delete => {
                                    app.delete_selected_header()
                                }
                                _ => (),
                            },
                            _ => (),
                        };
                        match key.code {
                            event::KeyCode::Esc => app.remove_new_header(),
                            event::KeyCode::Enter => {
                                if app.is_key_active() {
                                    app.add_header_key();
                                } else {
                                    app.add_header_value();
                                    app.remove_new_header();
                                }
                            }
                            event::KeyCode::Char('j') => app.increase_temp_idx(),
                            event::KeyCode::Char('k') => app.decrease_temp_idx(),
                            event::KeyCode::Tab => app.change_activity_selected_header(),
                            _ => (),
                        };
                    }
                    app::RequestTabs::Params(_, _) => {
                        if app.has_new_param() {
                            match key.code {
                                event::KeyCode::Char(x) => app.add_to_kv(x),
                                event::KeyCode::Backspace => {
                                    app.remove_from_kv();
                                }
                                event::KeyCode::Tab => app.change_active(),
                                _ => (),
                            };
                        };
                        match key.modifiers {
                            event::KeyModifiers::CONTROL => match key.code {
                                event::KeyCode::Char('n') => app.initiate_new_param(),
                                event::KeyCode::Char('d') | event::KeyCode::Delete => {
                                    app.delete_selected_param()
                                }
                                _ => (),
                            },
                            _ => (),
                        };
                        match key.code {
                            event::KeyCode::Esc => app.remove_new_param(),
                            event::KeyCode::Enter => {
                                if app.is_key_active() {
                                    app.add_param_key();
                                } else {
                                    app.add_param_value();
                                    app.remove_new_param();
                                }
                            }
                            event::KeyCode::Char('j') => app.increase_temp_idx(),
                            event::KeyCode::Char('k') => app.decrease_temp_idx(),
                            event::KeyCode::Tab => app.change_activity_selected_param(),
                            _ => (),
                        };
                    }
                    app::RequestTabs::Body(_, _) => (),
                },
            };
        };
    }
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let address = default_block("Address");
    let verb = default_block("Verb");
    let body = default_block("Response");
    let request_data = default_block("Request Data");

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

    let body_titles: Vec<Spans> = app
        .resp_tabs
        .resp_tabs
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at();
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();
    let body_tabs = Tabs::new(body_titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Response tabs"),
        )
        .select(app.resp_tabs.selected_idx)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    let l = layout::LayoutBuilder::default(f, app.has_new_header(), app.has_new_param());
    let data = Paragraph::new(app.address().clone().unwrap_or("".to_string()))
        .wrap(tui::widgets::Wrap { trim: true });
    let verb_str = Paragraph::new(app.verb()).wrap(tui::widgets::Wrap { trim: true });

    f.render_widget(tabs, l.req_tabs);
    f.render_widget(body_tabs, l.body_tabs);

    match app.selected_window {
        Windows::Address => {
            let address = address.border_type(tui::widgets::BorderType::Thick);
            let data = data.block(address);
            let verb = verb_str.block(verb);
            handle_response_data(app, f, body, &l);
            handle_request_data(app, f, request_data, &l);
            f.render_widget(verb, l.verb);
            f.render_widget(data, l.address);
        }
        Windows::Response => {
            let body = body.border_type(tui::widgets::BorderType::Thick);
            let data = data.block(address);
            let verb = verb_str.block(verb);
            handle_request_data(app, f, request_data, &l);
            f.render_widget(verb, l.verb);
            f.render_widget(data, l.address);
            handle_response_data(
                app,
                f,
                body.border_type(tui::widgets::BorderType::Thick),
                &l,
            );
        }
        Windows::Verb => {
            let verb = verb.border_type(tui::widgets::BorderType::Thick);
            let data = data.block(address);
            let verb = verb_str.block(verb);
            handle_request_data(app, f, request_data, &l);
            f.render_widget(verb, l.verb);
            f.render_widget(data, l.address);
            handle_response_data(app, f, body, &l);
        }
        Windows::RequestData => {
            let data = data.block(address);
            let verb = verb_str.block(verb);

            handle_request_data(
                app,
                f,
                request_data.border_type(tui::widgets::BorderType::Thick),
                &l,
            );
            handle_response_data(app, f, body, &l);
            f.render_widget(verb, l.verb);
            f.render_widget(data, l.address);
        }
    }
    if app.error_pop_up.0 {
        if let Some(e) = &app.error_pop_up.1 {
            error_popup(f, e, f.size());
        }
        return;
    }
    if app.show_environments {
        show_environments(f, &mut app)
    }
}

fn default_block(name: &str) -> Block {
    Block::default()
        .title(name)
        .style(Style::default().bg(Color::Black))
        .borders(Borders::ALL)
}

fn handle_request_data<B: Backend>(
    app: &App,
    f: &mut Frame<B>,
    b: Block,
    r: &layout::LayoutBuilder,
) {
    // Headers Table
    match app.req_tabs.selected {
        app::RequestTabs::Headers(_, _) => {
            if app.has_new_header() {
                if let Some(h) = &r.new_header {
                    let mut key_block = default_block("Key");
                    let mut value_block = default_block("Value");
                    if app.is_key_active() {
                        key_block = key_block.border_type(tui::widgets::BorderType::Thick);
                    } else {
                        value_block = value_block.border_type(tui::widgets::BorderType::Thick);
                    }
                    let k = Paragraph::new(app.new_headers()[0].clone())
                        .wrap(tui::widgets::Wrap { trim: true })
                        .block(key_block);
                    let v = Paragraph::new(app.new_headers()[1].clone())
                        .wrap(tui::widgets::Wrap { trim: true })
                        .block(value_block);

                    f.render_widget(k, h.key);
                    f.render_widget(v, h.value);
                };
            };
            let selected_style = Style::default().add_modifier(Modifier::BOLD);
            let normal_style = Style::default().bg(Color::Blue);
            if let Some(headers) = app.headers() {
                let rows = headers.iter().map(|item| {
                    let height = 1;
                    let cells = {
                        vec![
                            Cell::from(item.0.clone()),
                            Cell::from(item.1.clone()),
                            Cell::from(format!(
                                "{}",
                                if item.2.clone() { "Active" } else { "Inactive" }
                            )),
                        ]
                    };
                    Row::new(cells).height(height as u16).bottom_margin(0)
                });
                let t = Table::new(rows)
                    .block(b)
                    .highlight_style(selected_style)
                    .highlight_symbol(">> ")
                    .widths(&[
                        Constraint::Percentage(50),
                        Constraint::Length(30),
                        Constraint::Min(10),
                    ]);
                let state = &mut TableState::default();
                state.select(Some(app.temp_header_param_idx));
                f.render_stateful_widget(t, r.req_data, state);
            } else {
                f.render_widget(b, r.req_data);
            }
        }
        app::RequestTabs::Params(_, _) => {
            if app.has_new_param() {
                if let Some(h) = &r.new_header {
                    let mut key_block = default_block("Key");
                    let mut value_block = default_block("Value");
                    if app.is_key_active() {
                        key_block = key_block.border_type(tui::widgets::BorderType::Thick);
                    } else {
                        value_block = value_block.border_type(tui::widgets::BorderType::Thick);
                    }
                    let k = Paragraph::new(app.new_param()[0].clone())
                        .wrap(tui::widgets::Wrap { trim: true })
                        .block(key_block);
                    let v = Paragraph::new(app.new_param()[1].clone())
                        .wrap(tui::widgets::Wrap { trim: true })
                        .block(value_block);

                    f.render_widget(k, h.key);
                    f.render_widget(v, h.value);
                };
            };
            let selected_style = Style::default().add_modifier(Modifier::BOLD);
            let normal_style = Style::default().bg(Color::Blue);
            if let Some(headers) = app.params() {
                let headers = headers.clone();
                let rows = headers.iter().map(|item| {
                    let height = 1;
                    let cells = {
                        vec![
                            Cell::from(item.0.clone()),
                            Cell::from(item.1.clone()),
                            Cell::from(format!(
                                "{}",
                                if item.2.clone() { "Active" } else { "Inactive" }
                            )),
                        ]
                    };
                    Row::new(cells).height(height as u16).bottom_margin(0)
                });
                let t = Table::new(rows)
                    .block(b)
                    .highlight_style(selected_style)
                    .highlight_symbol(">> ")
                    .widths(&[
                        Constraint::Percentage(50),
                        Constraint::Length(30),
                        Constraint::Min(10),
                    ]);
                let state = &mut TableState::default();
                state.select(Some(app.temp_header_param_idx));
                f.render_stateful_widget(t, r.req_data, state);
            } else {
                f.render_widget(b, r.req_data);
            }
        }

        app::RequestTabs::Body(_, _) => f.render_widget(b, r.req_data),
    }
}

fn handle_response_data<B: Backend>(
    app: &App,
    f: &mut Frame<B>,
    b: Block,
    r: &layout::LayoutBuilder,
) {
    // Headers Table
    match app.resp_tabs.selected {
        app::ResponseTabs::Headers(_, _) => {
            let selected_style = Style::default().add_modifier(Modifier::BOLD);
            let normal_style = Style::default().bg(Color::Blue);
            if let Some(headers) = app.response_headers() {
                let rows = headers.iter().map(|item| {
                    let height = 1;
                    let cells = { vec![Cell::from(item.0.clone()), Cell::from(item.1.clone())] };
                    Row::new(cells).height(height as u16).bottom_margin(0)
                });
                let t = Table::new(rows)
                    .block(b)
                    .highlight_style(selected_style)
                    .highlight_symbol(">> ")
                    .widths(&[
                        Constraint::Percentage(50),
                        Constraint::Length(30),
                        Constraint::Min(1),
                    ]);
                f.render_stateful_widget(t, r.body, &mut TableState::default());
            } else {
                f.render_widget(b, r.body);
            }
        }
        app::ResponseTabs::Body(_, _) => {
            let resp =
                Paragraph::new(app.response_body().clone()).wrap(tui::widgets::Wrap { trim: true });
            f.render_widget(resp.block(b), r.body);
        }
    }
    let sc_block = default_block("status_code");
    let mut sc_p = Paragraph::new("");
    let sc = app.response_status_code();
    if sc != 0 {
        sc_p = Paragraph::new(format!("{}", sc))
            .alignment(tui::layout::Alignment::Center)
            .wrap(tui::widgets::Wrap { trim: true });
        if sc > 100 && sc < 400 {
            sc_p = sc_p.style(Style::default().fg(Color::Green));
        } else {
            sc_p = sc_p.style(Style::default().fg(Color::Red));
        }
    }
    f.render_widget(sc_p.block(sc_block), r.resp_status_code);
}

fn error_popup<B: Backend>(f: &mut Frame<B>, e: &app::Error, r: Rect) {
    let block = Block::default()
        .title("Error")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));
    let area = layout::centered_rect(60, 20, r);
    let msg = Paragraph::new(format!("{:?}", e))
        .wrap(tui::widgets::Wrap { trim: true })
        .block(block)
        .style(Style::default().fg(Color::Red));
    //f.render_widget(Clear, area); //this clears out the background
    f.render_widget(msg, area);
}

fn show_environments<B: Backend>(f: &mut Frame<B>, app: &mut App) {

}

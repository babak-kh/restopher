mod app;
mod components;
mod environments;
mod layout;
mod request;
mod response;
use components::{default_block, to_selected};
use std::io;
use tokio;
use tui_textarea::TextArea;

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
    widgets::{
        Block, Borders, Cell, Clear, List, ListItem, ListState, Paragraph, Row, Table, TableState,
        Tabs,
    },
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
                    _ => (),
                },
                _ => (),
            };
            match app.selected_main_window {
                app::MainWindows::RequestScr => {
                    match key.modifiers {
                        event::KeyModifiers::CONTROL => match key.code {
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
                                app.selected_main_window = app::MainWindows::EnvironmentScr;
                                app.initiate_temp_envs();
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
                    };
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
                            app::RequestTabs::Body(_, _) => {
                                match key.modifiers {
                                    event::KeyModifiers::CONTROL => match key.code {
                                        event::KeyCode::Char('n') => {
                                            app.change_body_kind();
                                            continue;
                                        }
                                        _ => (),
                                    },
                                    _ => (),
                                };
                                match key.code {
                                    event::KeyCode::Char(x) => {
                                        app.add_to_req_body(x);
                                    }
                                    event::KeyCode::Backspace => {
                                        app.remove_from_req_body();
                                    }
                                    event::KeyCode::Enter => {
                                        app.add_to_req_body('\n');
                                    }
                                    _ => (),
                                };
                            }
                        },
                        Windows::EnvSelection => match key.code {
                            event::KeyCode::Up => app.next_env(),
                            event::KeyCode::Down => app.pre_env(),
                            event::KeyCode::Esc => app.deselect_env(),
                            _ => (),
                        },
                    };
                }
                app::MainWindows::EnvironmentScr => {
                    match key.code {
                        event::KeyCode::Esc => {
                            if let Some(temp) = &mut app.temp_envs {
                                if temp.with_kv_insertion || temp.with_name_insertion {
                                    temp.with_name_insertion = false;
                                    temp.with_kv_insertion = false;
                                    continue;
                                }
                            }
                            app.selected_main_window = app::MainWindows::RequestScr;
                            app.clear_temp_envs()
                                .or_else(|e| {
                                    println!("{:?}", e);
                                    app.error_pop_up = (true, Some(e));
                                    Ok::<(), app::Error>(())
                                })
                                .unwrap();
                            continue;
                        }

                        event::KeyCode::Tab => {
                            match app.temp_envs {
                                Some(ref mut t) => {
                                    if t.with_kv_insertion {
                                        app.change_active_env_kv();
                                        continue;
                                    }
                                }
                                None => continue,
                            }
                            app.change_active_env_panel();
                        }
                        _ => (),
                    };
                    match key.modifiers {
                        event::KeyModifiers::CONTROL => match key.code {
                            event::KeyCode::Char('n') => {
                                if let Some(temp) = &mut app.temp_envs {
                                    match temp.sections {
                                        environments::EnvironmentSubSection::Name => {
                                            temp.with_name_insertion = true;
                                            continue;
                                        }
                                        environments::EnvironmentSubSection::KVs => {
                                            temp.with_kv_insertion = true;
                                            continue;
                                        }
                                    };
                                };
                            }
                            event::KeyCode::Char('d') => {
                                if let Some(temp) = &mut app.temp_envs {
                                    match temp.sections {
                                        environments::EnvironmentSubSection::Name => {
                                            temp.remove_name();
                                            continue;
                                        }
                                        environments::EnvironmentSubSection::KVs => {
                                            temp.remove_kv();
                                            continue;
                                        }
                                    };
                                }
                            }
                            event::KeyCode::Char('j') => {
                                if let Some(temp) = &mut app.temp_envs {
                                    match temp.sections {
                                        environments::EnvironmentSubSection::Name => {
                                            temp.selected += 1;
                                            if temp.selected >= temp.temp_envs.len() {
                                                temp.selected = 0;
                                            }
                                        }
                                        environments::EnvironmentSubSection::KVs => {
                                            if temp.temp_envs.len() > 0 {
                                                let t = &temp.temp_envs[temp.selected];
                                                temp.selected_kv += 1;
                                                if temp.selected_kv >= temp.temp_envs.len() {
                                                    temp.selected_kv = 0;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            event::KeyCode::Char('k') => {
                                if let Some(temp) = &mut app.temp_envs {
                                    match temp.sections {
                                        environments::EnvironmentSubSection::Name => {
                                            if temp.selected == 0 {
                                                temp.selected = temp.temp_envs.len() - 1;
                                            } else {
                                                temp.selected -= 1;
                                            }
                                        }
                                        environments::EnvironmentSubSection::KVs => {
                                            if temp.temp_envs.len() > 0 {
                                                let t = &temp.temp_envs[temp.selected];
                                                if temp.selected_kv == 0 {
                                                    temp.selected_kv = t.envs.len() - 1;
                                                } else {
                                                    temp.selected_kv -= 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => (),
                        },
                        _ => (),
                    };
                    match key.code {
                        event::KeyCode::Char(x) => {
                            if let Some(temp) = &mut app.temp_envs {
                                if temp.with_name_insertion {
                                    temp.name_insertion.push(x);
                                } else if temp.with_kv_insertion {
                                    if temp.kv_insertion.is_key_active {
                                        temp.kv_insertion.key.push(x);
                                    } else {
                                        temp.kv_insertion.value.push(x);
                                    }
                                }
                            }
                        }
                        event::KeyCode::Backspace => {
                            if let Some(temp) = &mut app.temp_envs {
                                if temp.with_name_insertion {
                                    temp.name_insertion.pop();
                                } else if temp.with_kv_insertion {
                                    if temp.kv_insertion.is_key_active {
                                        temp.kv_insertion.key.pop();
                                    } else {
                                        temp.kv_insertion.value.pop();
                                    }
                                }
                            }
                        }
                        event::KeyCode::Enter => {
                            if let Some(temp) = &mut app.temp_envs {
                                if temp.with_name_insertion {
                                    let name = temp.name_insertion.clone();
                                    app.new_environment(name);
                                } else if temp.with_kv_insertion {
                                    if temp.kv_insertion.is_key_active {
                                        app.change_active_env_kv()
                                    } else {
                                        let name = temp.temp_envs[temp.selected].name.clone();
                                        let k = temp.kv_insertion.key.clone();
                                        let v = temp.kv_insertion.value.clone();
                                        app.add_to_env_kv(name, k, v);
                                    }
                                }
                            }
                        }
                        _ => (),
                    }
                }
            };
            match key.code {
                event::KeyCode::Esc => {
                    if app.error_pop_up.0 {
                        app.error_pop_up = (false, None);
                    }
                }
                _ => (),
            }
        };
    }
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
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

    // component declarations
    let tabs = components::tabs(titles, "Request data tabs", app.req_tabs.selected_idx);
    let address = components::default_block("Address");
    let verb = components::default_block("Verb");
    let body = components::default_block("Response");
    let request_data = components::default_block("Request Data");
    let body_tabs = components::tabs(body_titles, "Response Tabs", app.resp_tabs.selected_idx);
    let env_block = components::default_block("Environment");
    let req_names = components::default_block("Requests");
    let mut body_kind_select = false;

    match app.req_tabs.selected {
        app::RequestTabs::Body(_, _) => body_kind_select = true,
        _ => (),
    }

    let l = layout::LayoutBuilder::default(
        f,
        app.has_new_header(),
        app.has_new_param(),
        app.has_new_env_name(),
        app.has_new_env_kv(),
        body_kind_select,
    );

    let data = Paragraph::new(app.address().clone().unwrap_or("".to_string()))
        .wrap(tui::widgets::Wrap { trim: true });
    let verb_str = Paragraph::new(app.verb()).wrap(tui::widgets::Wrap { trim: true });
    let env_text = match app.current_env_idx {
        Some(idx) => app.all_envs[idx].name.clone(),
        None => "".to_string(),
    };
    let env_text = Paragraph::new(env_text).wrap(tui::widgets::Wrap { trim: true });

    f.render_widget(tabs, l.req_tabs);
    f.render_widget(body_tabs, l.body_tabs);

    match app.selected_window {
        Windows::Address => {
            let address = to_selected(address);
            let data = data.block(address);
            let verb = verb_str.block(verb);
            let env_block = env_text.block(env_block);
            let req_names_tabs = components::tabs(app.get_req_names(), "Requests", app.current_request_idx);
            handle_response_data(app, f, bod:, &l);
            handle_request_data(app, f, request_data, &l);
            f.render_widget(verb, l.verb);
            f.render_widget(data, l.address);
            f.render_widget(env_block, l.env_selection);
        }
        Windows::Response => {
            let body = to_selected(body);
            let data = data.block(address);
            let verb = verb_str.block(verb);
            let env_block = env_text.block(env_block);
            handle_request_data(app, f, request_data, &l);
            f.render_widget(verb, l.verb);
            f.render_widget(data, l.address);
            f.render_widget(env_block, l.env_selection);
            handle_response_data(
                app,
                f,
                body.border_type(tui::widgets::BorderType::Thick),
                &l,
            );
        }
        Windows::Verb => {
            let verb = to_selected(verb);
            let data = data.block(address);
            let verb = verb_str.block(verb);
            let env_block = env_text.block(env_block);
            handle_request_data(app, f, request_data, &l);
            f.render_widget(verb, l.verb);
            f.render_widget(data, l.address);
            f.render_widget(env_block, l.env_selection);
            handle_response_data(app, f, body, &l);
        }
        Windows::RequestData => {
            let data = data.block(address);
            let verb = verb_str.block(verb);
            let env_block = env_text.block(env_block);
            f.render_widget(env_block, l.env_selection);
            handle_request_data(app, f, to_selected(request_data), &l);
            handle_response_data(app, f, body, &l);
            f.render_widget(verb, l.verb);
            f.render_widget(data, l.address);
        }
        Windows::EnvSelection => {
            let data = data.block(address);
            let verb = verb_str.block(verb);
            let env_block = env_text.block(to_selected(env_block));
            f.render_widget(env_block, l.env_selection);
            handle_request_data(app, f, request_data, &l);
            handle_response_data(app, f, body, &l);
            f.render_widget(verb, l.verb);
            f.render_widget(data, l.address);
        }
        Windows::ReqNames => todo!(),
    }
    match app.selected_main_window {
        app::MainWindows::EnvironmentScr => show_environments(f, &app, &l),
        _ => (),
    }
    if app.error_pop_up.0 {
        if let Some(e) = &app.error_pop_up.1 {
            error_popup(f, e, f.size());
        }
        return;
    }
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
                    let mut key_block = components::default_block("Key");
                    let mut value_block = components::default_block("Value");
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
                    let mut key_block = components::default_block("Key");
                    let mut value_block = components::default_block("Value");
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

        app::RequestTabs::Body(_, _) => {
            let mut kind_block = default_block("Content Type");
            match app.selected_window {
                Windows::RequestData => {
                    kind_block = to_selected(kind_block);
                }
                _ => (),
            }
            let body = app.req_body();
            let kind = Paragraph::new(body.kind.clone().to_string())
                .wrap(tui::widgets::Wrap { trim: true })
                .block(kind_block);

            let mut txt = TextArea::from(body.payload.unwrap_or("".to_string()).split("\n"));
            txt.set_cursor_line_style(Style::default());
            txt.move_cursor(tui_textarea::CursorMove::Bottom);
            txt.move_cursor(tui_textarea::CursorMove::End);
            txt.set_block(b);
            f.render_widget(txt.widget(), r.req_data);
            f.render_widget(kind, r.body_kind.unwrap());
        }
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
    let sc_block = components::default_block("status_code");
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
    f.render_widget(Clear, area);
    f.render_widget(msg, area);
}

fn show_environments<B: Backend>(f: &mut Frame<B>, app: &App, l: &layout::LayoutBuilder) {
    let mut environment_names = components::default_block("Environments");
    let mut environment_kvs = components::default_block("Variables");
    let mut new_name = default_block("new env");
    let mut new_k = default_block("key");
    let mut new_v = default_block("value");

    f.render_widget(Clear, l.el.all);
    if let Some(temp) = &app.temp_envs {
        match temp.sections {
            environments::EnvironmentSubSection::Name => {
                environment_names = to_selected(environment_names);
            }
            environments::EnvironmentSubSection::KVs => {
                environment_kvs = to_selected(environment_kvs);
            }
        }
        let mut current_values = &environments::Environment::default();
        if temp.temp_envs.len() > 0 {
            current_values = &temp.temp_envs[temp.selected]
        };
        if current_values.envs.len() > 0 {
            let selected_style = Style::default().add_modifier(Modifier::BOLD);
            let normal_style = Style::default().bg(Color::Blue);
            let rows = current_values.envs_to_show.iter().map(|item| {
                let height = 1;
                let cells = { vec![Cell::from(item[0].clone()), Cell::from(item[1].clone())] };
                Row::new(cells).height(height as u16).bottom_margin(0)
            });
            let t = Table::new(rows)
                .block(environment_kvs)
                .highlight_style(selected_style)
                .highlight_symbol(">> ")
                .widths(&[
                    Constraint::Percentage(50),
                    Constraint::Length(30),
                    Constraint::Min(1),
                ]);
            let mut state = &mut TableState::default();
            state.select(Some(temp.selected_kv));
            f.render_stateful_widget(t, l.el.kvs, &mut state);
        } else {
            f.render_widget(environment_kvs, l.el.kvs);
        }
        if temp.temp_envs.len() > 0 {
            let items: Vec<ListItem> = temp
                .temp_envs
                .iter()
                .map(|i| ListItem::new(i.name.clone()).style(Style::default().fg(Color::White)))
                .collect();

            let items = List::new(items)
                .block(environment_names)
                .highlight_style(
                    Style::default()
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");
            let mut state = &mut ListState::default();
            state.select(Some(temp.selected));
            f.render_stateful_widget(items, l.el.names, &mut state);
        } else {
            f.render_widget(environment_names, l.el.names);
        }
    }

    if let Some(temp) = &app.temp_envs {
        if temp.with_name_insertion {
            new_name = to_selected(new_name);
            f.render_widget(
                Paragraph::new(temp.name_insertion.clone()).block(new_name),
                l.el.new_name.unwrap_or_default(),
            );
        }
        if temp.with_kv_insertion {
            if let Some(kv) = &l.el.new_kv {
                if temp.kv_insertion.is_key_active {
                    new_k = to_selected(new_k);
                } else {
                    new_v = to_selected(new_v);
                }
                f.render_widget(
                    Paragraph::new(temp.kv_insertion.key.clone()).block(new_k),
                    kv.key,
                );
                f.render_widget(
                    Paragraph::new(temp.kv_insertion.value.clone()).block(new_v),
                    kv.value,
                );
            }
        }
    }
}

use crate::components::{default_block, tabs};
use crate::environments::Environment;
use crate::keys::keys::{Event, Key};
use crate::request::Request;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{List, ListItem, ListState, Paragraph},
    Frame,
};

enum focus {
    Env,
    Requests,
}
impl focus {
    fn next(&mut self) {
        match self {
            focus::Env => *self = focus::Requests,
            focus::Requests => *self = focus::Env,
        }
    }
}

pub struct RequestsComponent {
    is_focused: bool,
    focus: focus,
}

impl RequestsComponent {
    pub fn new(names: Vec<String>, idx: usize) -> Self {
        Self {
            is_focused: false,
            focus: focus::Requests,
        }
    }
    pub fn is_focused(&self) -> bool {
        self.is_focused
    }
    pub fn update(
        &mut self,
        requests: &mut Vec<Request>,
        request_index: &mut usize,
        environments: &mut Vec<Environment>,
        environment_index: &mut usize,
        event: &Event,
    ) {
        match event.key {
            Key::Tab => {
                self.focus.next();
                return;
            }
            _ => (),
        }
        match self.focus {
            focus::Env => match event.key {
                Key::Up => {
                    if *environment_index < environments.len() - 1 {
                        *environment_index += 1;
                    }
                    if *environment_index == environments.len() - 1 {
                        *environment_index = 0;
                    }
                }
                Key::Down => {
                    if *environment_index == environments.len() - 1 {
                        *environment_index = 0;
                        return;
                    }
                    *environment_index -= 1;
                }
                _ => (),
            },
            focus::Requests => match event.key {
                Key::Char('h') => {
                    if *request_index == requests.len() - 1 {
                        *request_index = 0;
                        return;
                    }
                    *request_index -= 1;
                }
                Key::Char('l') => {
                    if *request_index < requests.len() - 1 {
                        *request_index += 1;
                    }
                    if *request_index == requests.len() - 1 {
                        *request_index = 0;
                    }
                }
                _ => (),
            },
        }
    }
    pub fn focus(&mut self) {
        self.is_focused = true;
    }
    pub fn lose_focus(&mut self) {
        self.is_focused = false;
    }
    pub fn gain_focus(&mut self) {
        self.is_focused = true;
    }
    pub fn blur(&mut self) {
        self.is_focused = false;
    }
    pub fn draw(
        &self,
        f: &mut Frame,
        names: Vec<String>,
        env_name: String,
        selected: usize,
        rect: Rect,
    ) {
        let chunks =
            Layout::horizontal(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
                .split(rect);
        f.render_widget(
            tabs(
                names.iter().map(|t| Span::from(t.to_string())).collect(),
                "Requests",
                selected,
                self.is_focused && matches!(self.focus, focus::Requests),
            ),
            chunks[0],
        );
        f.render_widget(
            Paragraph::new(env_name).block(default_block(
                "Environment",
                self.is_focused && matches!(self.focus, focus::Env),
            )),
            chunks[1],
        );
    }
}

use crate::components::{default_block, tabs};
use crate::environments::Environment;
use crate::keys::keys::{Event, Key, Modifier};
use crate::layout::centered_rect;
use crate::request::Request;
use ratatui::widgets::Clear;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    text::Span,
    widgets::Paragraph,
    Frame,
};

use super::PopUpComponent;

enum Focus {
    Env,
    Requests,
}
impl Focus {
    fn next(&mut self) {
        match self {
            Focus::Env => *self = Focus::Requests,
            Focus::Requests => *self = Focus::Env,
        }
    }
}

pub enum Action {
    RequestRemoved,
    RequestIndexChanged,
    RequestCreated,
}

pub struct RequestsComponent {
    is_focused: bool,
    focus: Focus,
    popup: Option<PopUpComponent>,
}

impl RequestsComponent {
    pub fn new() -> Self {
        Self {
            is_focused: false,
            focus: Focus::Requests,
            popup: None,
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
    ) -> Option<Action> {
        if let Some(popup) = &mut self.popup {
            let result = popup.update(event);
            if result.1 {
                return None;
            };
            self.popup = None;
            if let Some(rename) = result.0 {
                requests[*request_index].set_name(rename);
            };
            return None;
        };
        match event.key {
            Key::Tab => {
                self.focus.next();
                return None;
            }
            _ => (),
        }
        match self.focus {
            Focus::Env => self.handle_env_update(environments, environment_index, event),
            Focus::Requests => self.handle_request_update(requests, request_index, event),
        }
    }
    fn handle_request_update(
        &mut self,
        requests: &mut Vec<Request>,
        request_index: &mut usize,
        event: &Event,
    ) -> Option<Action> {
        if let Some(modifier) = &event.modifier {
            match modifier {
                Modifier::Control => match event.key {
                    Key::Char('d') => {
                        if requests.len() == 1 {
                            None
                        } else {
                            requests.remove(*request_index);
                            if *request_index == requests.len() {
                                *request_index -= 1;
                            }
                            Some(Action::RequestRemoved)
                        }
                    }
                    _ => None,
                },
                Modifier::Alt => match event.key {
                    Key::Char('r') => {
                        self.popup = Some(PopUpComponent::new(
                            "Rename".to_string(),
                            "Rename Request".to_string(),
                        ));
                        None
                    }
                    _ => None,
                },
                _ => None,
            }
        } else {
            match event.key {
                Key::Char('h') => {
                    if *request_index == requests.len() - 1 {
                        *request_index = 0;
                        Some(Action::RequestIndexChanged)
                    } else {
                        *request_index -= 1;
                        Some(Action::RequestIndexChanged)
                    }
                }
                Key::Char('l') => {
                    if *request_index < requests.len() - 1 {
                        *request_index += 1;
                        Some(Action::RequestIndexChanged)
                    } else if *request_index == requests.len() - 1 {
                        *request_index = 0;
                        Some(Action::RequestIndexChanged)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
    }
    fn handle_env_update(
        &mut self,
        environments: &mut Vec<Environment>,
        environment_index: &mut usize,
        event: &Event,
    ) -> Option<Action> {
        match event.key {
            Key::Down => {
                if *environment_index < environments.len() {
                    *environment_index += 1;
                }
                if *environment_index == environments.len() {
                    *environment_index = 0;
                }
                None
            }
            Key::Up => {
                if *environment_index == 0 {
                    *environment_index = environments.len() - 1;
                } else {
                    *environment_index -= 1;
                }
                None
            }
            _ => None,
        }
    }
    pub fn lose_focus(&mut self) {
        self.is_focused = false;
    }
    pub fn gain_focus(&mut self) {
        self.is_focused = true;
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
                Some("Requests"),
                selected,
                self.is_focused && matches!(self.focus, Focus::Requests),
            ),
            chunks[0],
        );
        f.render_widget(
            Paragraph::new(env_name).block(default_block(
                Some("Environment"),
                self.is_focused && matches!(self.focus, Focus::Env),
            )),
            chunks[1],
        );
        if let Some(popup) = &self.popup {
            let r = centered_rect(60, 20, f.area());
            f.render_widget(Clear, r);
            popup.draw(f, r);
        }
    }
}

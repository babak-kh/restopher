mod address;
mod view;

use super::default_block;
use crate::{
    keys::keys::{Event, Key, Modifier},
    request::Request,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Paragraph, Wrap},
    Frame,
};
use view::Focus;

pub struct AddressBarComponent {
    focus: Focus,
    is_focused: bool,
}

impl AddressBarComponent {
    pub fn new() -> Self {
        AddressBarComponent {
            focus: Focus::Address,
            is_focused: false,
        }
    }
    pub fn is_focused(&self) -> bool {
        self.is_focused
    }
    pub fn gain_focus(&mut self) {
        self.is_focused = true;
    }
    pub fn lose_focus(&mut self) {
        self.is_focused = false;
    }
    pub fn update(&mut self, req: &mut Request, event: &Event) {
        match event.key {
            Key::Tab => self.focus.next(),
            Key::Up => {
                if matches!(self.focus, Focus::Verb) {
                    req.verb_up()
                }
            }
            Key::Down => {
                if matches!(self.focus, Focus::Verb) {
                    req.verb_down()
                }
            }
            Key::Char(x) => {
                if matches!(self.focus, Focus::Address) {
                    req.add_to_address(x);
                }
            }
            Key::Backspace => {
                if matches!(self.focus, Focus::Address) {
                    req.remove_from_address();
                }
            }
            _ => (),
        }
    }
    pub fn draw(&self, f: &mut Frame, req: &Request, rect: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(rect);
        f.render_widget(
            Paragraph::new(req.verb().to_string())
                .block(default_block(
                    Some("Verb"),
                    self.is_focused && matches!(self.focus, Focus::Verb),
                ))
                .wrap(Wrap { trim: true }),
            chunks[0],
        );
        f.render_widget(
            Paragraph::new(req.address().as_str())
                .block(default_block(
                    Some("Address"),
                    self.is_focused && matches!(self.focus, Focus::Address),
                ))
                .wrap(Wrap { trim: true }),
            chunks[1],
        );
    }
}

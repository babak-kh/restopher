mod address;
mod draw;
mod verb;
mod view;

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

use super::default_block;

pub struct AddressBarComponent {
    focus: Focus,
    is_focused: bool,
}

impl AddressBarComponent {
    pub fn new() -> Self {
        AddressBarComponent {
            focus: Focus::None,
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
    pub fn update(&self, req: &mut Request, event: &Event) {
        match &self.focus {
            Focus::Address => {
                if let Some(modifier) = &event.modifier {
                    match modifier {
                        Modifier::Control => todo!(),
                        Modifier::Shift => todo!(),
                        Modifier::Alt => todo!(),
                    }
                }
                match event.key {
                    Key::Char(x) => req.add_to_address(x),
                    Key::Backspace => req.remove_from_address(),
                    _ => (),
                }
            }
            Focus::Verb => {
                if let Some(modifier) = &event.modifier {
                    match modifier {
                        Modifier::Control => todo!(),
                        Modifier::Shift => todo!(),
                        Modifier::Alt => todo!(),
                    }
                }
                match event.key {
                    Key::Up => req.verb_up(),
                    Key::Down => req.verb_down(),
                    _ => (),
                }
            }
            Focus::None => (),
        }
    }
    pub fn draw(&self, f: &mut Frame, req: &Request, rect: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(rect);
        f.render_widget(
            Paragraph::new(req.verb.to_string())
                .block(default_block("Verb", self.is_focused))
                .wrap(Wrap { trim: true }),
            chunks[0],
        );
        f.render_widget(
            Paragraph::new(req.address.as_str())
                .block(default_block("Address", self.is_focused))
                .wrap(Wrap { trim: true }),
            chunks[1],
        );
    }
}

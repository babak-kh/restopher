mod address;
use copypasta::{ClipboardContext, ClipboardProvider};
mod view;

use super::default_block;
use crate::{
    components::text_box::TextBox,
    keys::keys::{is_ctrl_v, Event, Key},
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
    address_bar_view: TextBox,
}

impl AddressBarComponent {
    pub fn new() -> Self {
        AddressBarComponent {
            focus: Focus::Address,
            is_focused: false,
            address_bar_view: TextBox::new(),
        }
    }
    pub fn from(request: &Request) -> Self {
        AddressBarComponent {
            focus: Focus::Address,
            is_focused: false,
            address_bar_view: TextBox::from(request.address()),
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
            _ => {
                if matches!(self.focus, Focus::Address) {
                    if is_ctrl_v(event) {
                        let mut ctx = ClipboardContext::new().unwrap();
                        self.address_bar_view
                            .add_to_buffer(ctx.get_contents().unwrap());
                    } else {
                        self.address_bar_view.update(event);
                    }
                    let content = self.address_bar_view.get_content();
                    req.set_address(content);
                }
            }
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
                .style(req.verb().style())
                .wrap(Wrap { trim: true }),
            chunks[0],
        );
        self.address_bar_view.draw(
            f,
            chunks[1],
            "Address",
            self.is_focused && matches!(self.focus, Focus::Address),
        );
    }
}

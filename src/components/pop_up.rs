use crate::{
    components::text_box::TextBox,
    keys::keys::{Event, Key},
};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[derive(Debug)]
pub struct PopUpComponent {
    title: String,
    input: TextBox,
}

impl PopUpComponent {
    pub fn new(title: String, _: String) -> Self {
        Self {
            title,
            input: TextBox::new(),
        }
    }
    pub fn update(&mut self, event: &Event) -> (Option<String>, bool) {
        match event.key {
            Key::Char(_) => {
                self.input.update(event);
                return (None, true);
            }
            Key::Enter => {
                return (Some(self.input.get_content()), false);
            }
            Key::Esc => {
                return (None, false);
            }
            Key::Backspace => {
                self.input.update(event);
                return (None, true);
            }
            _ => {
                return (None, true);
            }
        }
    }
    pub fn draw(&self, f: &mut Frame, rect: Rect) {
        Layout::default()
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(rect)
            .iter()
            .enumerate()
            .for_each(|(i, r)| {
                let title = match i {
                    0 => self.title.as_str(),
                    _ => &self.input.get_content().clone(),
                };
                f.render_widget(
                    Paragraph::new(title)
                        .block(Block::default().borders(Borders::ALL))
                        .alignment(Alignment::Center),
                    *r,
                );
            });
    }
}

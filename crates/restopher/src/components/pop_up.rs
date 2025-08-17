use crate::{
    components::text_box::TextBox,
    keys::keys::{Event, Key},
};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph},
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
        let margin = 1;
        let container = Rect {
            x: rect.left() - margin,
            y: rect.top() - margin,
            width: rect.right() - rect.left() + (2 * margin),
            height: rect.bottom() - rect.top() + (2 * margin),
        };
        f.render_widget(Clear, container);
        f.render_widget(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .title(Span::styled(
                    self.title.as_str(),
                    Style::default().fg(Color::White),
                ))
                .title_alignment(Alignment::Center)
                .add_modifier(Modifier::BOLD),
            container,
        );
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

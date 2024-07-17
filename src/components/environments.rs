use super::default_block;
use ratatui::{layout::Rect, widgets::Paragraph, Frame};

pub struct EnvironmentsComponent {
    focused: bool,
}

impl EnvironmentsComponent {
    pub fn new() -> Self {
        EnvironmentsComponent { focused: false }
    }
    pub fn draw(&self, f: &mut Frame, name: String, rect: Rect) {
        f.render_widget(
            Paragraph::default().block(default_block(&name, self.focused)),
            rect,
        );
    }
}

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::layout;

pub fn error_popup(f: &mut Frame, e: &crate::app::Error, r: Rect) {
    let block = Block::default()
        .title("Error")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));
    let area = layout::centered_rect(60, 20, r);
    let msg = Paragraph::new(format!("{:?}", e))
        .wrap(Wrap { trim: true })
        .block(block)
        .style(Style::default().fg(Color::Red));
    f.render_widget(Clear, area);
    f.render_widget(msg, area);
}

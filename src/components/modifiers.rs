use ratatui::{widgets::Block, style::{Color, Style}};

pub fn to_selected(b: Block) -> Block {
    b.border_style(Style::default().fg(Color::Rgb(240, 134, 110)))
}

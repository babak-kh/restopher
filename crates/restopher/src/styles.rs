use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders},
};

pub fn cursor_like_span<'a>(c: char) -> Span<'a> {
    Span::raw(c.to_string()).style(
        Style::default()
            .add_modifier(Modifier::UNDERLINED)
            .underline_color(Color::Blue),
    )
}

pub fn selected_box<'a>() -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightGreen))
}

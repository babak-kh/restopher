use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

pub fn cursor_like_span<'a>(c: char) -> Span<'a> {
    Span::raw(c.to_string()).style(
        Style::default()
            .add_modifier(Modifier::UNDERLINED)
            .underline_color(Color::Blue),
    )
}

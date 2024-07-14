use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Tabs};

pub fn default_block(name: &str, is_focused: bool) -> Block {
    Block::default()
        .title(Span::styled(name, Style::default().fg(Color::White)))
        .borders(Borders::ALL)
        .border_style({
            if is_focused {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::White)
            }
        })
}

pub fn tabs<'a>(
    titles: Vec<Span<'a>>,
    name: &'a str,
    selected: usize,
    is_focused: bool,
) -> Tabs<'a> {
    Tabs::new(titles)
        .block(default_block(name, is_focused))
        .select(selected)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Yellow),
        )
}

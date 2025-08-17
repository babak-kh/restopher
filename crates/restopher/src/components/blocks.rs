use std::borrow::Cow;

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Tabs};

pub fn default_block(name: Option<&str>, is_focused: bool) -> Block {
    let b = Block::default()
        .borders(Borders::ALL)
        .border_style({
            if is_focused {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::White)
            }
        })
        .border_type(ratatui::widgets::BorderType::Rounded);
    if let Some(name) = name {
        b.title(Span::styled(name, Style::default().fg(Color::White)))
    } else {
        b
    }
}

pub fn tabs<'a>(
    titles: Vec<Span<'a>>,
    name: Option<&'a str>,
    selected: usize,
    is_focused: bool,
) -> Tabs<'a> {
    Tabs::new(
        titles
            .into_iter()
            .map(|mut t| {
                if t.content == "" {
                    t.content = Cow::Borrowed("Untitled");
                }
                return t;
            })
            .collect::<Vec<Span<'a>>>(),
    )
    .block(default_block(name, is_focused))
    .select(selected)
    .style(Style::default().fg(Color::White))
    .highlight_style(
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Yellow),
    )
}

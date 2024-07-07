use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Span};
use ratatui::widgets::{Block, Borders, Tabs};

pub fn default_block(name: &str) -> Block {
    Block::default()
        .title(Span::styled(name, Style::default().fg(Color::White)))
        .style(Style::default().bg(Color::Black))
        .borders(Borders::ALL)
}

pub fn tabs<'a>(titles: Vec<Span<'a>>, name: &'a str, selected: usize) -> Tabs<'a> {
    Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(name))
        .select(selected)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        )
}


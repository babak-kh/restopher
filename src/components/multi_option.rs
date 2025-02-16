use crate::components::default_block;
use crate::keys::keys::{Event, Key};
use crate::layout::centered_rect;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub struct MultiOptionWidget {
    title: String,
    options: Vec<String>,
    selected_idx: usize,
}

impl MultiOptionWidget {
    pub fn new(options: Vec<String>) -> Self {
        Self {
            options,
            selected_idx: 0,
            title: String::from(""),
        }
    }
    pub fn update(&mut self, event: &Event) -> Option<String> {
        match event.key {
            Key::Down => {
                if self.selected_idx < self.options.len() - 1 {
                    self.selected_idx += 1;
                }
            }
            Key::Up => {
                if self.selected_idx > 0 {
                    self.selected_idx -= 1;
                }
            }
            Key::Enter => {
                return Some(self.options[self.selected_idx].to_owned());
            }
            _ => {}
        }
        None
    }
    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        f.render_widget(Clear, area);
        let centered = centered_rect(50, 50, area);
        let chunks = Layout::vertical(
            (0..self.options.len())
                .map(|_| Constraint::Max(3))
                .collect::<Vec<Constraint>>(),
        )
        .split(centered);
        let margin = 1;
        let container = Rect {
            x: chunks[0].left() - margin,
            y: chunks[0].top() - margin,
            width: chunks[0].right() - chunks[0].left() + (2 * margin),
            // width: (chunks[chunks.len() - 1].right().bottom()),
            height: chunks[chunks.len() - 1].bottom() - chunks[0].top() + (2 * margin),
        };
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
        for (i, option) in self.options.iter().enumerate() {
            f.render_widget(
                {
                    Paragraph::new(Text::from(option.to_owned()).style(Style::default())).block({
                        if self.selected_idx == i {
                            default_block(None, true)
                        } else {
                            default_block(None, false)
                        }
                    })
                },
                chunks[i],
            )
        }
    }
}

use ratatui::{
    layout::{Alignment, Constraint, Layout},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::{
    keys::keys::{Event, Key},
    layout::centered_rect,
    styles::selected_box,
};

enum State {
    Yes,
    No,
}

pub struct YesNoPopupComponent<'a> {
    state: State,
    msg: &'a str,
}

impl<'a> YesNoPopupComponent<'a> {
    pub fn new(msg: &'a str) -> Self {
        Self {
            state: State::Yes,
            msg,
        }
    }
    pub fn update(&mut self, event: &Event) -> Option<bool> {
        match event.key {
            Key::Tab => {
                self.state = match self.state {
                    State::Yes => State::No,
                    State::No => State::Yes,
                };
            }
            Key::Enter => {
                return Some(matches!(self.state, State::Yes));
            }
            _ => {}
        }
        None
    }
    pub fn draw(&self, frame: &mut Frame) {
        let rect = centered_rect(40, 20, frame.area());
        let chunks =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rect);
        let yes_no = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);
        frame.render_widget(Clear, rect);
        frame.render_widget(
            Paragraph::new(self.msg)
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Center),
            chunks[0],
        );
        frame.render_widget(
            Paragraph::new("Yes")
                .block(if matches!(self.state, State::Yes) {
                    selected_box()
                } else {
                    Block::default().borders(Borders::ALL)
                })
                .alignment(Alignment::Center),
            yes_no[0],
        );
        frame.render_widget(
            Paragraph::new("No")
                .block({
                    if matches!(self.state, State::No) {
                        selected_box()
                    } else {
                        Block::default().borders(Borders::ALL)
                    }
                })
                .alignment(Alignment::Center),
            yes_no[1],
        );
    }
}

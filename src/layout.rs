use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::Frame;

pub struct LayoutBuilder {
    pub verb: Rect,
    pub address: Rect,
    pub body: Rect,
}

impl LayoutBuilder {
    pub fn default<B: Backend>(base: &mut Frame<B>) -> Self {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
            .split(base.size());
        let chunks_h = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(5), Constraint::Percentage(95)])
            .split(chunks[0]);
        LayoutBuilder {
            verb: chunks_h[0],
            address: chunks_h[1],
            body: chunks[1],
        }
    }
}

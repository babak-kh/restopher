use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub struct AppLayout {
    pub requests: Rect,
    pub address_verb: Rect,
    pub request: Rect,
    pub response: Rect,
}
impl AppLayout {
    pub fn new(r: Rect) -> Self {
        let chunks = Layout::default()
            .flex(Flex::Center)
            .margin(0)
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(5),  // req names
                Constraint::Percentage(5),  // verb + address
                Constraint::Percentage(45), // req tab + headers/body/params
                Constraint::Percentage(45), // resp tab + headers/body
            ])
            .split(r);
        AppLayout {
            requests: chunks[0],
            address_verb: chunks[1],
            request: chunks[2],
            response: chunks[3],
        }
    }
}

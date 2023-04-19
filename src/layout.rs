use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::Frame;

pub struct LayoutBuilder {
    pub verb: Rect,
    pub address: Rect,
    pub body_tabs: Rect,
    pub body: Rect,
    pub req_tabs: Rect,
    pub req_data: Rect,
    pub new_header: Option<KV>,
}
pub struct KV {
    pub key: Rect,
    pub value: Rect,
}

impl LayoutBuilder {
    pub fn default<B: Backend>(
        base: &mut Frame<B>,
        with_new_header: bool,
        with_new_param: bool,
    ) -> Self {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(10), // verb + address
                Constraint::Percentage(10), // req tabs
                Constraint::Percentage(40), // req headers/body/params
                Constraint::Percentage(10), // resp headers/body
                Constraint::Percentage(30), // response
            ])
            .split(base.size());
        let chunks_h = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(5), Constraint::Percentage(95)])
            .split(chunks[0]);
        let mut req_data = chunks[2];
        let mut new_header: Option<KV> = None;
        if with_new_header | with_new_param {
            let new_header_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
                .margin(0)
                .split(chunks[2]);
            req_data = new_header_layout[0];
            let kv_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(new_header_layout[1]);
            new_header = Some(KV {
                key: kv_layout[0],
                value: kv_layout[1],
            });
        }
        LayoutBuilder {
            verb: chunks_h[0],
            address: chunks_h[1],
            body: chunks[4],
            req_tabs: chunks[1],
            body_tabs: chunks[3],
            req_data,
            new_header,
        }
    }
}

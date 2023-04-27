use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Margin, Rect};
use tui::Frame;

pub struct KV {
    pub key: Rect,
    pub value: Rect,
}
impl Default for KV {
    fn default() -> Self {
        KV {
            key: Rect::default(),
            value: Rect::default(),
        }
    }
}
pub struct LayoutBuilder {
    pub verb: Rect,
    pub address: Rect,
    pub body_tabs: Rect,
    pub resp_status_code: Rect,
    pub body: Rect,
    pub req_tabs: Rect,
    pub req_data: Rect,
    pub new_header: Option<KV>,
    pub env_selection: Rect,
    pub el: EnvironmentLayout,
}
pub struct EnvironmentLayout {
    pub all: Rect,
    pub names: Rect,
    pub new_name: Option<Rect>,
    pub new_kv: Option<KV>,
    pub kvs: Rect,
}
impl EnvironmentLayout {
    pub fn new<B: Backend>(f: &mut Frame<B>, with_new_name: bool, with_new_kv: bool) -> Self {
        let all = f.size().inner(&Margin {
            vertical: 10,
            horizontal: 10,
        });

        let mut names = Rect::default();
        let mut kvs = Rect::default();
        let mut new_name = None;
        let mut new_kv = None;

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(all);
        names = chunks[0];
        kvs = chunks[1];
        if with_new_name {
            let nn = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
                .split(chunks[0]);
            new_name = Some(nn[1]);
            names = nn[0];
        };
        if with_new_kv {
            let nn = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
                .split(chunks[1]);
            let kv_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(nn[1]);
            new_kv = Some(KV {
                key: kv_layout[0],
                value: kv_layout[1],
            });
            kvs = nn[0];
        };
        EnvironmentLayout {
            names,
            kvs,
            new_kv,
            new_name,
            all,
        }
    }
}

impl LayoutBuilder {
    pub fn default<B: Backend>(
        base: &mut Frame<B>,
        with_new_header: bool,
        with_new_param: bool,
        with_new_name: bool,
        with_new_kv: bool,
    ) -> Self {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(6),  // verb + address
                Constraint::Percentage(6),  // req tabs
                Constraint::Percentage(44), // req headers/body/params
                Constraint::Percentage(6),  // resp headers/body tabs
                Constraint::Percentage(38), // response
            ])
            .split(base.size());
        let chunks_h = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(5),
                Constraint::Percentage(5),
                Constraint::Percentage(95),
            ])
            .split(chunks[0]);
        let status_code_body_tabs = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
            .split(chunks[3]);
        let body_tabs = status_code_body_tabs[0];
        let resp_status_code = status_code_body_tabs[1];
        let mut req_data = chunks[2];
        let mut new_header: Option<KV> = None;
        if with_new_header | with_new_param {
            let new_header_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
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
            address: chunks_h[2],
            env_selection: chunks_h[1],
            body: chunks[4],
            req_tabs: chunks[1],
            body_tabs,
            resp_status_code,
            req_data,
            new_header,
            el: EnvironmentLayout::new(base, with_new_name, with_new_kv),
        }
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
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

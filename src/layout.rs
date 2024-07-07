use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout, Margin, Rect};
use ratatui::Frame;

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
pub struct CollectionsLayout {
    pub all: Rect,
    pub names: Rect,
    pub new_name: Option<Rect>,
    pub payload: Rect,
}
impl CollectionsLayout {
    pub fn new(f: &mut Frame, has_new_name: bool, has_new_collection: bool) -> Self {
        let all = f.size().inner(&Margin {
            vertical: 20,
            horizontal: 20,
        });

        let mut names = Rect::default();
        let mut kvs = Rect::default();
        let mut new_name = None;

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(all);
        if has_new_name | has_new_collection {
            let new_name_collection = Layout::default()
                .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
                .direction(Direction::Vertical)
                .split(chunks[0]);
            names = new_name_collection[0];
            new_name = Some(new_name_collection[1]);
        };
        names = chunks[0];
        kvs = chunks[1];
        Self {
            all,
            names,
            new_name,
            payload: kvs,
        }
    }
}

pub struct LayoutBuilder {
    pub req_names: Rect,
    pub verb: Rect,
    pub address: Rect,
    pub body_tabs: Rect,
    pub resp_status_code: Rect,
    pub body: Rect,
    pub req_tabs: Rect,
    pub req_data: Rect,
    pub resp_data: Rect,
    pub new_header: Option<KV>,
    pub env_selection: Rect,
    pub body_kind: Option<Rect>,
    pub el: EnvironmentLayout,
    pub cl: CollectionsLayout,
}
pub struct EnvironmentLayout {
    pub all: Rect,
    pub names: Rect,
    pub new_name: Option<Rect>,
    pub new_kv: Option<KV>,
    pub kvs: Rect,
}
impl EnvironmentLayout {
    pub fn new(f: &mut Frame, with_new_name: bool, with_new_kv: bool) -> Self {
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
    pub fn default(
        base: &mut Frame,
        with_new_header: bool,
        with_new_param: bool,
        with_new_name: bool,
        with_new_kv: bool,
        body_selected: bool,
        with_new_collection: bool,
        with_new_req_name: bool,
    ) -> Self {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(6),  // req names
                Constraint::Percentage(6),  // verb + address
                Constraint::Percentage(6),  // req tabs
                Constraint::Percentage(41), // req headers/body/params
                Constraint::Percentage(6),  // resp headers/body tabs
                Constraint::Percentage(35), // response
            ])
            .split(base.size());
        let chunks_h = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(5),
                Constraint::Percentage(5),
                Constraint::Percentage(95),
            ])
            .split(chunks[1]);
        let status_code_body_tabs = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
            .split(chunks[4]);
        let body_tabs = status_code_body_tabs[0];
        let resp_status_code = status_code_body_tabs[1];
        let mut req_data = chunks[3];
        let mut new_header: Option<KV> = None;
        if with_new_header | with_new_param {
            let new_header_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
                .margin(0)
                .split(chunks[3]);
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
        let mut body_kind = None;
        if body_selected {
            let body_kind_split = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(15), Constraint::Percentage(26)])
                .split(chunks[3]);
            body_kind = Some(body_kind_split[0]);
            req_data = body_kind_split[1];
        };

        LayoutBuilder {
            req_names: chunks[0],
            verb: chunks_h[0],
            address: chunks_h[2],
            env_selection: chunks_h[1],
            body: chunks[5],
            req_tabs: chunks[2],
            body_tabs,
            resp_status_code,
            req_data,
            resp_data: chunks[6],
            new_header,
            body_kind,
            el: EnvironmentLayout::new(base, with_new_name, with_new_kv),
            cl: CollectionsLayout::new(base, with_new_req_name, with_new_collection),
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

pub struct AppLayout {
    pub requests: Rect,
    pub req_tabs: Rect,
    pub request: RequestsLayout,
    pub resp_tabs: Rect,
}
impl AppLayout {
    pub fn new(r: Rect) -> Self {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(6),  // req names
                Constraint::Percentage(6),  // verb + address
                Constraint::Percentage(6),  // req tabs
                Constraint::Percentage(41), // req headers/body/params
                Constraint::Percentage(6),  // resp headers/body tabs
                Constraint::Percentage(35), // response
            ])
            .split(r);
        AppLayout {
            requests: chunks[0],
            req_tabs: chunks[2],
            request: RequestsLayout {
                verb_address: chunks[1],
                request_data: chunks[3],
                response_data: chunks[5],
            },
            resp_tabs: chunks[4],
        }
    }
}
pub struct RequestsLayout {
    pub verb_address: Rect,
    pub request_data: Rect,
    pub response_data: Rect,
}

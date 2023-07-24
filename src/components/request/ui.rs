use tui::{layout::Rect, widgets::{Widget, Paragraph}};

use crate::components::default_block;

use super::HttpVerb;

pub fn verb(v: HttpVerb) -> impl Widget {
    Paragraph::new(v.to_string()).block(default_block("verb"))
}

pub fn address(addr: String) -> impl Widget {
    Paragraph::new(addr).block(default_block("address"))
}

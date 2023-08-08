use tui::{layout::Rect, widgets::{Widget, Paragraph, Block}};

use crate::components::default_block;

use super::HttpVerb;

pub fn verb<'a>(v: HttpVerb) -> Paragraph<'a> {
    Paragraph::new(v.to_string()).block(default_block("verb"))
}

pub fn address<'a>(addr: String) -> Paragraph<'a> {
    Paragraph::new(addr).block(default_block("address"))
}

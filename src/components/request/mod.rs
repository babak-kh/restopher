mod bundle;
mod request;
mod response;
mod ui;
mod view;

use std::collections::HashMap;

pub use bundle::ReqBundle;
pub use request::HttpVerb;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::{
    keys::keys::{Event, Key, Modifier},
    layout,
    utils::app_state::{Section, State},
};

use super::default_block;

pub const ADDRESS: Section = "address";
pub const VERB: Section = "verb";
pub const PARAMS: Section = "params";
pub const HEADERS: Section = "headers";
pub const BODY: Section = "body";

pub struct RequestController {}

impl RequestController {
    pub fn new() -> Self {
        RequestController {}
    }
    pub fn handle(&self, event: Event, req: &mut ReqBundle) -> Result<(), crate::app::Error> {
        if let Some(modifier) = event.modifier {
            match modifier {
                Modifier::Control => todo!(),
                Modifier::Shift => todo!(),
                Modifier::Alt => todo!(),
            }
        }
        Ok(())
    }
    pub fn render<B: Backend>(
        f: &mut Frame<B>,
        req: &ReqBundle,
        rect: layout::RequestsLayout,
        state: &State,
    ) {
        req.render(f, rect, state);
    }
}

mod bundle;
mod request;
mod response;
mod ui;
mod view;

use std::collections::HashMap;

pub use bundle::ReqBundle;
pub use request::HttpVerb;
use ratatui::{
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
pub const RESPONSE_BODY: Section = "response_body";
pub const RESPONSE_HEADERS: Section = "response_headers";
pub const RequestTabs: Section = "request_tabs";
pub const ResponseTabs: Section = "response_tabs";

pub struct RequestController {}

impl RequestController {
    pub fn new() -> Self {
        RequestController {}
    }
    pub fn handle(
        &self,
        event: Event,
        req: &mut ReqBundle,
        state: &State,
    ) -> Result<(), crate::app::Error> {
        match state.last().sub() {
            ADDRESS => {
                if let Some(modifier) = event.modifier {
                    match modifier {
                        Modifier::Control => todo!(),
                        Modifier::Shift => todo!(),
                        Modifier::Alt => todo!(),
                    }
                }
                match event.key {
                    Key::Char(x) => req.add_to_address(x),
                    Key::Backspace => req.remove_from_address(),
                    _ => (),
                }
            }
            VERB => {
                if let Some(modifier) = event.modifier {
                    match modifier {
                        Modifier::Control => todo!(),
                        Modifier::Shift => todo!(),
                        Modifier::Alt => todo!(),
                    }
                }
                match event.key {
                    Key::Up => req.verb_up(),
                    Key::Down => req.verb_down(),
                    _ => (),
                }
            }
            HEADERS => {
                if let Some(modifier) = event.modifier {
                    match modifier {
                        Modifier::Control => match event.key {
                            Key::Char('n') => {
                                if !req.view.has_new_header() {
                                    req.view.initiate_new_header();
                                };
                                return Ok(());
                            }
                            Key::Char('d') => {
                                if !req.view.has_new_header() {
                                    req.delete_selected_header();
                                }
                            }
                            Key::Char(' ') => {
                                if !req.view.has_new_header() {
                                    req.active_deactive_header()
                                };
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }
                match event.key {
                    Key::Esc => {
                        if req.view.has_new_header() {
                            req.view.remove_new_header();
                        }
                    }
                    Key::Char(x) => {
                        if req.view.has_new_header() {
                            req.view.add_to_active_header(x);
                        };
                    }
                    Key::Backspace => {
                        if req.view.has_new_header() {
                            req.view.remove_from_active_header();
                        };
                    }
                    Key::Enter => {
                        if req.view.has_new_header() {
                            req.add_to_header();
                            req.view.remove_new_header();
                        };
                    }
                    Key::Tab => {
                        if req.view.has_new_header() {
                            req.view.change_active_header();
                        };
                    }
                    Key::Down => {
                        if !req.view.has_new_header() {
                            let len = req.headers_len() as i32;
                            req.view
                                .header_idx_ops(view::IndexOperation::Increase(1), len);
                        };
                    }
                    Key::Up => {
                        if !req.view.has_new_header() {
                            let len = req.headers_len() as i32;
                            req.view
                                .header_idx_ops(view::IndexOperation::Decrease(1), len)
                        };
                    }
                    _ => (),
                }
            }
            PARAMS => {
                if let Some(modifier) = event.modifier {
                    match modifier {
                        Modifier::Control => match event.key {
                            Key::Char('n') => {
                                if !req.view.has_new_param() {
                                    req.view.initiate_new_param();
                                };
                                return Ok(());
                            }
                            Key::Char('d') => {
                                if !req.view.has_new_param() {
                                    req.delete_selected_param();
                                }
                            }
                            Key::Char('a') => {
                                if !req.view.has_new_param() {
                                    req.active_deactive_param()
                                };
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }
                match event.key {
                    Key::Esc => {
                        if req.view.has_new_param() {
                            req.view.remove_new_param();
                        }
                    }
                    Key::Char(x) => {
                        if req.view.has_new_param() {
                            req.view.add_to_active_param(x);
                        };
                    }
                    Key::Backspace => {
                        if req.view.has_new_param() {
                            req.view.remove_from_active_param();
                        };
                    }
                    Key::Enter => {
                        if req.view.has_new_param() {
                            req.add_to_param();
                            req.view.remove_new_param();
                        };
                    }
                    Key::Tab => {
                        if req.view.has_new_param() {
                            req.view.change_active_param();
                        };
                    }
                    Key::Down => {
                        if !req.view.has_new_param() {
                            let len = req.params_len() as i32;
                            req.view
                                .param_idx_ops(view::IndexOperation::Increase(1), len);
                        };
                    }
                    Key::Up => {
                        if !req.view.has_new_param() {
                            let len = req.params_len() as i32;
                            req.view
                                .param_idx_ops(view::IndexOperation::Decrease(1), len)
                        };
                    }
                    _ => (),
                }
            }
            _ => (),
        }
        Ok(())
    }
    pub fn render(f: &mut Frame, req: &ReqBundle, rect: layout::RequestsLayout, state: &State) {
        req.render(f, rect, state);
    }
}

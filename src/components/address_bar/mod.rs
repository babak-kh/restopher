mod address;
mod draw;
mod verb;
mod view;

use crate::{
    keys::keys::{Event, Key, Modifier},
    request::Request,
};
use view::Focus;

pub struct AddressBarComponent {
    focus: Focus,
    is_active: bool,
}

impl AddressBarComponent {
    pub fn new() -> Self {
        AddressBarComponent {
            focus: Focus::None,
            is_active: false,
        }
    }
    pub fn is_active(&self) -> bool {
        self.is_active
    }
    pub fn update(&self, req: &mut Request, event: &Event) {
        match &self.focus {
            Focus::Address => {
                if let Some(modifier) = &event.modifier {
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
            Focus::Verb => {
                if let Some(modifier) = &event.modifier {
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
            Focus::None => (),
        }
    }
}

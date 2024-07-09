use crate::keys::keys::{Event, Key, Modifier};

#[derive(Clone, Debug)]
pub enum MainWindows {
    Main,
    Environments,
    Settings,
    Collections,
}

pub enum ChangeEvent {
    ChangeRequestTab,
    ChangeResponseTab,
    SaveRequest,
    NewRequest,
    PreRequest,
    NextRequest,
    CallRequest,
    NoChange,
}

pub fn key_registry(event: &Event, main_window: &MainWindows) -> ChangeEvent {
    match main_window {
        MainWindows::Main => match event {
            Event {
                modifier: Some(Modifier::Control),
                key: Key::Char('t'),
            } => {
                return ChangeEvent::ChangeRequestTab;
            }
            Event {
                modifier: Some(Modifier::Control),
                key: Key::Char('p'),
            } => {
                return ChangeEvent::CallRequest;
            }
            Event {
                modifier: Some(Modifier::Control),
                key: Key::Char('r'),
            } => {
                return ChangeEvent::ChangeResponseTab;
            }
            Event {
                modifier: Some(Modifier::Control),
                key: Key::Char('s'),
            } => {
                return ChangeEvent::SaveRequest;
            }
            Event {
                modifier: Some(Modifier::Control),
                key: Key::Right,
            } => {
                return ChangeEvent::NextRequest;
            }
            Event {
                modifier: Some(Modifier::Control),
                key: Key::Left,
            } => {
                return ChangeEvent::PreRequest;
            }
            Event {
                modifier: Some(Modifier::Control),
                key: Key::Char('w'),
            } => {
                return ChangeEvent::NewRequest;
            }
            _ => return ChangeEvent::NoChange,
        },
        _ => return ChangeEvent::NoChange,
    }
}

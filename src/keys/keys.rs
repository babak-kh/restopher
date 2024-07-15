use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};

#[derive(PartialEq, Eq, Debug)]
pub enum Modifier {
    Control,
    Shift,
    Alt,
}
#[derive(PartialEq, Eq, Debug)]
pub enum Key {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Backspace,
    Esc,
    Enter,
    Tab,
}
#[derive(PartialEq, Eq, Debug)]
pub struct Event {
    pub modifier: Option<Modifier>,
    pub key: Key,
}

pub const OPEN_COLLECTIONS: &Event = &Event {
    modifier: Some(Modifier::Control),
    key: Key::Char('c'),
};

pub const CLOSE_COLLECTIONS: &Event = &Event {
    modifier: None,
    key: Key::Esc,
};

pub const OPEN_ENVIRONMENTS: &Event = &Event {
    modifier: Some(Modifier::Control),
    key: Key::Char('e'),
};

pub const CLOSE_ENVIRONMENTS: &Event = &Event {
    modifier: None,
    key: Key::Esc,
};

pub const NAV_UP: &Event = &Event {
    modifier: Some(Modifier::Control),
    key: Key::Char('k'),
};
pub const NAV_DOWN: &Event = &Event {
    modifier: Some(Modifier::Control),
    key: Key::Char('j'),
};
pub const NAV_LEFT: &Event = &Event {
    modifier: Some(Modifier::Control),
    key: Key::Char('h'),
};
pub const NAV_RIGHT: &Event = &Event {
    modifier: Some(Modifier::Control),
    key: Key::Char('l'),
};

pub fn transform(key: KeyEvent) -> Event {
    let mut modi: Option<Modifier> = None;
    let k: Key;

    match key.modifiers {
        KeyModifiers::ALT => modi = Some(Modifier::Alt),
        KeyModifiers::CONTROL => modi = Some(Modifier::Control),
        KeyModifiers::SHIFT => modi = Some(Modifier::Shift),
        _ => (),
    }
    match key.code {
        KeyCode::Backspace => k = Key::Backspace,
        KeyCode::Enter => k = Key::Enter,
        KeyCode::Left => k = Key::Left,
        KeyCode::Right => k = Key::Right,
        KeyCode::Up => k = Key::Up,
        KeyCode::Down => k = Key::Down,
        KeyCode::Home => todo!(),
        KeyCode::End => todo!(),
        KeyCode::PageUp => todo!(),
        KeyCode::PageDown => todo!(),
        KeyCode::Tab => k = Key::Tab,
        KeyCode::BackTab => todo!(),
        KeyCode::Delete => todo!(),
        KeyCode::Insert => todo!(),
        KeyCode::F(_) => todo!(),
        KeyCode::Char(x) => k = Key::Char(x),
        KeyCode::Null => todo!(),
        KeyCode::Esc => k = Key::Esc, //todo!(),
        KeyCode::CapsLock => todo!(),
        KeyCode::ScrollLock => todo!(),
        KeyCode::NumLock => todo!(),
        KeyCode::PrintScreen => todo!(),
        KeyCode::Pause => todo!(),
        KeyCode::Menu => todo!(),
        KeyCode::KeypadBegin => todo!(),
        KeyCode::Media(_) => todo!(),
        KeyCode::Modifier(_) => todo!(),
    }
    Event {
        modifier: modi,
        key: k,
    }
}
fn is_modifier(e: &Event, ms: Vec<Modifier>) -> bool {
    if let Some(em) = &e.modifier {
        for m in ms {
            if *em == m {
                return true;
            }
        }
    }
    false
}
fn is_key(e: &Event, ks: Vec<Key>) -> bool {
    for m in ks {
        if e.key == m {
            return true;
        }
    }
    false
}

pub fn is_quit(e: &Event) -> bool {
    if let Some(modi) = &e.modifier {
        match modi {
            Modifier::Alt => match e.key {
                Key::Char(x) => {
                    if x == 'q' {
                        return true;
                    }
                }
                _ => (),
            },
            _ => (),
        };
    }
    false
}
pub fn is_navigation(e: &Event) -> bool {
    e == NAV_UP || e == NAV_DOWN || e == NAV_LEFT || e == NAV_RIGHT
}

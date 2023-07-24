use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};

use crate::utils::app_state::State;

pub enum Modifier {
    Control,
    Shift,
    Alt,
}
pub enum Key {
    Char(char),
    Up,
    Down,
    Left,
    Right,
}

pub struct Event<'a> {
    pub modifier: Option<Modifier>,
    pub key: Key,
    pub state: &'a State,
}

pub fn transform(key: KeyEvent, state: &mut State) -> Event {
    let mut modi: Option<Modifier> = None;
    let mut k: Key;

    match key.modifiers {
        KeyModifiers::ALT => modi = Some(Modifier::Alt),
        KeyModifiers::CONTROL => modi = Some(Modifier::Control),
        KeyModifiers::SHIFT => modi = Some(Modifier::Shift),
        _ => (),
    }
    match key.code {
        KeyCode::Backspace => todo!(),
        KeyCode::Enter => todo!(),
        KeyCode::Left => k = Key::Left,
        KeyCode::Right => k = Key::Right,
        KeyCode::Up => k = Key::Up,
        KeyCode::Down => k = Key::Down,
        KeyCode::Home => todo!(),
        KeyCode::End => todo!(),
        KeyCode::PageUp => todo!(),
        KeyCode::PageDown => todo!(),
        KeyCode::Tab => todo!(),
        KeyCode::BackTab => todo!(),
        KeyCode::Delete => todo!(),
        KeyCode::Insert => todo!(),
        KeyCode::F(_) => todo!(),
        KeyCode::Char(x) => k = Key::Char(x),
        KeyCode::Null => todo!(),
        KeyCode::Esc => todo!(),
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
        state: state,
    }
}

pub fn is_quit(e: &Event) -> bool {
    if let Some(modi) = &e.modifier {
        match modi {
            Modifier::Control => match e.key {
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

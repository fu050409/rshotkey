use anyhow::Result;
use rdev::EventType;
use std::time::Duration;

use crate::exception::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Key {
    KeyUnknown,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    KeyLeftCtrl,
    KeyRightCtrl,
    KeyShift,
    KeyAlt,
    KeyEnter,
    KeyEscape,
    KeyBackspace,
    KeyTab,
    KeySpace,
    KeyMinus,
    KeyEqual,
    KeyLeftBracket,
    KeyRightBracket,
    KeyBackslash,
    KeySemicolon,
    KeyApostrophe,
    KeyGrave,
    KeyComma,
    KeyDot,
    KeySlash,
    KeyCapsLock,
    KeyF1,
    KeyF2,
    KeyF3,
    KeyF4,
    KeyF5,
    KeyF6,
    KeyF7,
    KeyF8,
    KeyF9,
    KeyF10,
    KeyF11,
    KeyF12,
    KeyPrintScreen,
    KeyScrollLock,
    KeyPause,
    KeyInsert,
    KeyHome,
    KeyPageUp,
    KeyDelete,
    KeyEnd,
    KeyPageDown,
    KeyRight,
    KeyLeft,
    KeyDown,
    KeyUp,
    KeyNumLock,
}

impl From<EventType> for Key {
    fn from(value: EventType) -> Self {
        match value {
            EventType::KeyPress(key) => match key {
                rdev::Key::Alt => Key::KeyAlt,
                rdev::Key::AltGr => todo!(),
                rdev::Key::Backspace => Key::KeyBackspace,
                rdev::Key::CapsLock => Key::KeyCapsLock,
                rdev::Key::ControlLeft => Key::KeyLeftCtrl,
                rdev::Key::ControlRight => Key::KeyRightCtrl,
                rdev::Key::Delete => Key::KeyDelete,
                rdev::Key::DownArrow => Key::KeyDown,
                rdev::Key::End => Key::KeyEnd,
                rdev::Key::Escape => Key::KeyEscape,
                rdev::Key::F1 => Key::KeyF1,
                rdev::Key::F2 => Key::KeyF2,
                rdev::Key::F3 => Key::KeyF3,
                rdev::Key::F4 => Key::KeyF4,
                rdev::Key::F5 => Key::KeyF5,
                rdev::Key::F6 => Key::KeyF6,
                rdev::Key::F7 => Key::KeyF7,
                rdev::Key::F8 => Key::KeyF8,
                rdev::Key::F9 => Key::KeyF9,
                rdev::Key::F10 => Key::KeyF10,
                rdev::Key::F11 => Key::KeyF11,
                rdev::Key::F12 => Key::KeyF12,
                rdev::Key::Home => Key::KeyHome,
                rdev::Key::LeftArrow => Key::KeyLeft,
                rdev::Key::MetaLeft => todo!(),
                rdev::Key::MetaRight => todo!(),
                rdev::Key::PageDown => todo!(),
                rdev::Key::PageUp => Key::KeyPageUp,
                rdev::Key::Return => Key::KeyEnter,
                rdev::Key::RightArrow => todo!(),
                rdev::Key::ShiftLeft => todo!(),
                rdev::Key::ShiftRight => todo!(),
                rdev::Key::Space => Key::KeySpace,
                rdev::Key::Tab => todo!(),
                rdev::Key::UpArrow => todo!(),
                rdev::Key::PrintScreen => todo!(),
                rdev::Key::ScrollLock => todo!(),
                rdev::Key::Pause => todo!(),
                rdev::Key::NumLock => todo!(),
                rdev::Key::BackQuote => todo!(),
                rdev::Key::Num1 => Key::Key1,
                rdev::Key::Num2 => Key::Key2,
                rdev::Key::Num3 => Key::Key3,
                rdev::Key::Num4 => Key::Key4,
                rdev::Key::Num5 => Key::Key5,
                rdev::Key::Num6 => Key::Key6,
                rdev::Key::Num7 => Key::Key7,
                rdev::Key::Num8 => Key::Key8,
                rdev::Key::Num9 => Key::Key9,
                rdev::Key::Num0 => Key::Key0,
                rdev::Key::Minus => Key::KeyMinus,
                rdev::Key::Equal => Key::KeyEqual,
                rdev::Key::LeftBracket => Key::KeyLeftBracket,
                rdev::Key::RightBracket => Key::KeyRightBracket,
                rdev::Key::SemiColon => Key::KeySemicolon,
                rdev::Key::Quote => todo!(),
                rdev::Key::BackSlash => Key::KeyBackslash,
                rdev::Key::IntlBackslash => Key::KeyBackslash,
                rdev::Key::KeyA => Key::KeyA,
                rdev::Key::KeyB => Key::KeyB,
                rdev::Key::KeyC => Key::KeyC,
                rdev::Key::KeyD => Key::KeyD,
                rdev::Key::KeyE => Key::KeyE,
                rdev::Key::KeyF => Key::KeyF,
                rdev::Key::KeyG => Key::KeyG,
                rdev::Key::KeyH => Key::KeyH,
                rdev::Key::KeyI => Key::KeyI,
                rdev::Key::KeyJ => Key::KeyJ,
                rdev::Key::KeyK => Key::KeyK,
                rdev::Key::KeyL => Key::KeyL,
                rdev::Key::KeyM => Key::KeyM,
                rdev::Key::KeyN => Key::KeyN,
                rdev::Key::KeyO => Key::KeyO,
                rdev::Key::KeyP => Key::KeyP,
                rdev::Key::KeyQ => Key::KeyQ,
                rdev::Key::KeyR => Key::KeyR,
                rdev::Key::KeyS => Key::KeyS,
                rdev::Key::KeyT => Key::KeyT,
                rdev::Key::KeyU => Key::KeyU,
                rdev::Key::KeyV => Key::KeyV,
                rdev::Key::KeyW => Key::KeyW,
                rdev::Key::KeyX => Key::KeyX,
                rdev::Key::KeyY => Key::KeyY,
                rdev::Key::KeyZ => Key::KeyZ,
                rdev::Key::Comma => Key::KeyComma,
                rdev::Key::Dot => Key::KeyDot,
                rdev::Key::Slash => Key::KeySlash,
                rdev::Key::Insert => Key::KeyInsert,
                rdev::Key::KpReturn => todo!(),
                rdev::Key::KpMinus => todo!(),
                rdev::Key::KpPlus => todo!(),
                rdev::Key::KpMultiply => todo!(),
                rdev::Key::KpDivide => todo!(),
                rdev::Key::Kp0 => todo!(),
                rdev::Key::Kp1 => todo!(),
                rdev::Key::Kp2 => todo!(),
                rdev::Key::Kp3 => todo!(),
                rdev::Key::Kp4 => todo!(),
                rdev::Key::Kp5 => todo!(),
                rdev::Key::Kp6 => todo!(),
                rdev::Key::Kp7 => todo!(),
                rdev::Key::Kp8 => todo!(),
                rdev::Key::Kp9 => todo!(),
                rdev::Key::KpDelete => todo!(),
                rdev::Key::Function => todo!(),
                rdev::Key::Unknown(code) => match code {
                    _ => Key::KeyUnknown,
                },
            },
            _ => Key::KeyUnknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BindKey {
    pub keys: Vec<Key>,
    pub delay_time: Duration,
}

impl BindKey {
    pub fn new(keys: Vec<Key>) -> Self {
        Self {
            keys,
            delay_time: Duration::from_secs(0),
        }
    }

    pub fn delay(&mut self, time: Duration) -> Self {
        self.delay_time = time;
        self.to_owned()
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }
}

impl From<Vec<EventType>> for BindKey {
    fn from(value: Vec<EventType>) -> Self {
        let mut keys: Vec<Key> = vec![];
        for event in value {
            keys.push(event.into());
        }
        Self {
            keys,
            delay_time: Duration::from_secs(0),
        }
    }
}

impl From<EventType> for BindKey {
    fn from(value: EventType) -> Self {
        Self {
            keys: vec![value.into()],
            delay_time: Duration::from_secs(0),
        }
    }
}

impl Into<Vec<BindKey>> for BindKey {
    fn into(self) -> Vec<BindKey> {
        vec![self]
    }
}

impl Into<Vec<BindKey>> for &BindKey {
    fn into(self) -> Vec<BindKey> {
        vec![self.clone()]
    }
}

impl IntoIterator for BindKey {
    type Item = Key;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.keys.into_iter()
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KeySet {
    pub bind_keys: Vec<BindKey>,
}

impl KeySet {
    pub fn new(key: Vec<BindKey>) -> Self {
        Self { bind_keys: key }
    }

    pub fn bind(&mut self, bind_key: BindKey) -> Self {
        self.bind_keys.push(bind_key);
        self.to_owned()
    }

    pub fn len(&self) -> usize {
        let mut count: usize = 0;
        for key in &self.bind_keys {
            count += key.len();
        }
        count
    }

    pub fn count(&self, idx: usize) -> Result<usize> {
        if idx >= self.bind_keys.len() {
            return Err(Error::OutOfIndex.into());
        }
        Ok(self.bind_keys[idx].len())
    }

    pub fn last(&self) -> Option<&BindKey> {
        self.bind_keys.last()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl From<Vec<BindKey>> for KeySet {
    fn from(value: Vec<BindKey>) -> Self {
        Self { bind_keys: value }
    }
}

impl From<BindKey> for KeySet {
    fn from(value: BindKey) -> Self {
        Self {
            bind_keys: vec![value],
        }
    }
}

impl From<Vec<EventType>> for KeySet {
    fn from(value: Vec<EventType>) -> Self {
        let mut keys: Vec<Key> = vec![];
        for event in value {
            keys.push(event.into());
        }
        KeySet::default().bind(BindKey::new(keys))
    }
}

impl From<EventType> for KeySet {
    fn from(value: EventType) -> Self {
        KeySet::default().bind(BindKey::new(vec![value.into()]))
    }
}

impl IntoIterator for KeySet {
    type Item = BindKey;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.bind_keys.into_iter()
    }
}

use anyhow::Result;
use rdev::EventType;
use std::time::Duration;

use crate::exception::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct BindKey {
    pub keys: Vec<EventType>,
    pub delay_time: Duration,
}

impl BindKey {
    pub fn new(keys: Vec<EventType>) -> Self {
        Self {
            keys,
            delay_time: Duration::from_secs(0),
        }
    }

    pub fn delay(&mut self, time: Duration) -> Self {
        self.delay_time = time;
        self.to_owned()
    }

    pub fn len(&mut self) -> usize {
        self.keys.len()
    }
}

impl From<Vec<EventType>> for BindKey {
    fn from(value: Vec<EventType>) -> Self {
        Self {
            keys: value,
            delay_time: Duration::from_secs(0),
        }
    }
}

impl From<EventType> for BindKey {
    fn from(value: EventType) -> Self {
        Self {
            keys: vec![value],
            delay_time: Duration::from_secs(0),
        }
    }
}

impl Into<Vec<BindKey>> for BindKey {
    fn into(self) -> Vec<BindKey> {
        vec![self]
    }
}

impl IntoIterator for BindKey {
    type Item = EventType;
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

    pub fn len(&mut self) -> usize {
        let mut count: usize = 0;
        for key in &mut self.bind_keys {
            count += key.len();
        }
        count
    }

    pub fn count(&mut self, idx: usize) -> Result<usize> {
        if idx >= self.bind_keys.len() {
            return Err(Error::OutOfIndex.into());
        }
        Ok(self.bind_keys[idx].len())
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
        KeySet::default().bind(BindKey::new(value))
    }
}

impl From<EventType> for KeySet {
    fn from(value: EventType) -> Self {
        KeySet::default().bind(BindKey::new(vec![value]))
    }
}

impl IntoIterator for KeySet {
    type Item = BindKey;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.bind_keys.into_iter()
    }
}

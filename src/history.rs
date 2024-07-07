use rdev::Event;

#[derive(Debug, Clone, Default)]
pub struct History {
    capacity: usize,
    entries: Vec<Event>,
}

// unsss

impl History {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: Vec::new(),
        }
    }

    pub fn push(&mut self, event: Event) {
        self.entries.push(event);
    }

    pub fn last_n(&self, n: usize) -> &[Event] {
        &self.entries[self.entries.len().saturating_sub(n)..]
    }

    pub fn last(&self) -> Option<&Event> {
        self.entries.last()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn clean(&mut self) {
        self.entries = self.last_n(self.capacity).to_vec();
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::*;

    #[test]
    fn test_history_record() {
        let mut history = History::new(1024);
        history.push(Event {
            time: SystemTime::now(),
            name: None,
            event_type: rdev::EventType::KeyPress(rdev::Key::ControlLeft),
        });
        history.push(Event {
            time: SystemTime::now(),
            name: None,
            event_type: rdev::EventType::KeyRelease(rdev::Key::ControlLeft),
        });
        assert_eq!(history.entries.len(), 2);
    }

    #[test]
    fn test_history_last_slice() {
        let mut history = History::new(1024);
        let event = Event {
            time: SystemTime::now(),
            name: None,
            event_type: rdev::EventType::KeyPress(rdev::Key::ControlLeft),
        };
        for _ in 0..10 {
            history.push(event.clone());
        }
        assert_eq!(history.last_n(5).len(), 5);
    }

    #[test]
    fn test_history_last_n() {
        let mut history = History::new(1024);
        let event = Event {
            time: SystemTime::now(),
            name: None,
            event_type: rdev::EventType::KeyPress(rdev::Key::ControlLeft),
        };
        let last = Event {
            time: SystemTime::now(),
            name: None,
            event_type: rdev::EventType::KeyPress(rdev::Key::KeyA),
        };
        for _ in 0..10 {
            history.push(event.clone());
        }
        history.push(last.clone());
        assert_eq!(history.last().unwrap(), &last);
    }
}

use crate::rorshach::event_type::EventType;
use std::path::PathBuf;
use std::fmt;

#[derive(Clone)]
pub struct Event {
    event_type: EventType,
    path: PathBuf,
}

impl Event {
    pub fn new(event_type: EventType, path: PathBuf) -> Self {
        Event{event_type: event_type, path: path}
    }

    pub fn get_event_type(&self) -> EventType {
        self.event_type
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.event_type, self.path.display())
    }
}
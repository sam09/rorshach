use crate::rorshach::event_type::EventType;
use std::path::PathBuf;
use std::fmt;

#[derive(Clone)]
pub struct Event {
    event_type: EventType,
    old_path: Option<PathBuf>,
    new_path: Option<PathBuf>,
}

impl Event {
    pub fn new(event_type: EventType, old_path: Option<PathBuf>, new_path: Option<PathBuf>) -> Self {
        Event{event_type, old_path, new_path}
    }

    pub fn get_event_type(&self) -> EventType {
        self.event_type
    }

    pub fn get_old_path(&self) -> &Option<PathBuf> {
        &self.old_path
    }

    pub fn get_new_path(&self) -> &Option<PathBuf> {
        &self.new_path
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.event_type, self.old_path, self.new_path)
    }
}
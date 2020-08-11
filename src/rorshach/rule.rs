use serde::Deserialize;
use std::fmt;
use crate::rorshach::event_type::EventType;

#[derive(Debug, Deserialize, Clone)]
pub struct Rule {
    event_type: EventType,
    file_pattern: String,
    cmd: String,
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.event_type, self.file_pattern, self.cmd)
    }
}

impl Rule {
    pub fn get_cmd(&self) -> &str {
        &self.cmd
    }
    pub fn get_file_pattern(&self) -> &str {
        &self.file_pattern
    }
    pub fn get_event_type(&self) -> EventType {
        self.event_type
    }
}
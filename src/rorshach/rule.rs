use serde::Deserialize;
use std::fmt;
use crate::rorshach::event::Event;

#[derive(Debug, Deserialize)]
pub struct Rule {
    event: Event,
    file_pattern: String,
    cmd: String,
}


impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.event, self.file_pattern, self.cmd)
    }
}

impl Rule {
    pub fn get_cmd(&self) -> &str {
        &self.cmd
    }
    pub fn get_file_pattern(&self) -> &str {
        &self.file_pattern
    }
    pub fn get_event(&self) -> Event {
        self.event
    }
}
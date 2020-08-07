use pub_sub::Subscription;
use crate::rorshach::event::Event;
use crate::rorshach::rule::Rule;
use regex::Regex;
use log::error;
use std::process::Command;

pub struct Consumer {
    receiver: Subscription<Event>,
    rule: Rule,
    dir: String,
}

impl Consumer {

    pub fn new(receiver: Subscription<Event>, rule: &Rule, dir: String) -> Self {
        Consumer{ receiver, rule: rule.clone(), dir}
    }

    fn exec_rule(&self, event: Event) {
        let old_path_str = match event.get_old_path() {
            Some(path) => path.to_string_lossy().to_string(),
            None => {
                error!("No path found for event: {}", &event);
                return;
            }
        };

        let new_path_str = match event.get_new_path() {
            Some(path) => path.to_string_lossy().to_string(),
            None => "".to_string(),
        };

        let re_str = format!("^{}$", self.rule.get_file_pattern());
        let re = match Regex::new(&re_str) {
            Err(err) => {
                error!("Ill formed pattern found {}: {}", old_path_str, err);
                return;
            }
            Ok(re) => re,
        };
        if re.is_match(&old_path_str) {
            if let Err(e) = Command::new("sh")
                .arg("-c")
                .arg(self.rule.get_cmd())
                .env("FULLPATH", &old_path_str)
                .env("NEWFULLPATH", &new_path_str)
                .env("BASEDIR", &self.dir)
                .spawn() {
                    error!("Spawning command {} on {} failed: {}", self.rule.get_cmd(), &old_path_str, e);
                }
        }
    }

    pub async fn consume(&self) {
        match self.receiver.recv() {    
            Ok(event) => {
                if event.get_event_type() == self.rule.get_event_type() {
                    self.exec_rule(event);
                }
            },
            Err(err) => {
                error!("Error occurred while consuming event: {}", err)
            }
        };
    }
}
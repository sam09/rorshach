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
        Consumer{ receiver: receiver, rule: rule.clone(), dir: dir}
    }

    fn exec_rule(&self, event: Event) {
        let path_str = match event.get_old_path() {
            Some(path) => path.to_string_lossy().to_string(),
            None => {
                error!("No path found for event: {}", &event);
                return;
            }
        };
        let re_str = format!("^{}$", self.rule.get_file_pattern());
        let re = match Regex::new(&re_str) {
            Err(err) => {
                error!("Ill formed pattern found {}: {}", path_str, err);
                return;
            }
            Ok(re) => re,
        };
        if re.is_match(&path_str) {
            match Command::new("sh")
                .arg("-c")
                .arg(self.rule.get_cmd())
                .env("FULLPATH", &path_str)
                .env("BASEDIR", &self.dir)
                .spawn() {
                    Err(e) => {
                        error!("Spawning command {} on {} failed: {}", self.rule.get_cmd(), &path_str, e);
                    },
                    _ => (),
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
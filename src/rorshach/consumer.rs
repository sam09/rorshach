use pub_sub::Subscription;
use crate::rorshach::event::Event;
use crate::rorshach::rule::Rule;
use regex::Regex;
use log::error;
use std::process::Command;
use anyhow::{Result, Context, bail};

pub struct Consumer {
    receiver: Subscription<Event>,
    rule: Rule,
    dir: String,
}

impl Consumer {

    pub fn new(receiver: Subscription<Event>, rule: &Rule, dir: String) -> Self {
        Consumer{ receiver, rule: rule.clone(), dir}
    }

    fn exec_rule(&self, event: &Event) -> Result<()> {
        let old_path_str = match event.get_old_path() {
            Some(path) => path.to_string_lossy().to_string(),
            None => bail!("No path found for event: {}", &event),
        };

        let new_path_str = match event.get_new_path() {
            Some(path) => path.to_string_lossy().to_string(),
            None => "".to_string(),
        };

        let re_str = format!("^{}$", self.rule.get_file_pattern());
        let re = Regex::new(&re_str).with_context(|| format!("Invalid pattern {}", re_str))?;
        if re.is_match(&old_path_str) {
            Command::new("sh")
                .arg("-c")
                .arg(self.rule.get_cmd())
                .env("FULLPATH", &old_path_str)
                .env("NEWFULLPATH", &new_path_str)
                .env("BASEDIR", &self.dir)
                .spawn()
                .with_context(|| format!("Spawning command {} on {} failed", self.rule.get_cmd(), &old_path_str))?;
        }
        Ok(())
    }

    pub async fn consume(&self) {
        match self.receiver.recv() {    
            Ok(event) => {
                if event.get_event_type() == self.rule.get_event_type() {
                    if let Err(e) = self.exec_rule(&event) {
                        error!("Error executing {} for {}: {}", self.rule, &event, e);
                    }
                }
            },
            Err(err) => {
                error!("Error occurred while consuming event: {}", err)
            }
        };
    }
}
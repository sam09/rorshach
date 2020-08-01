
use crate::rorshach::rule::Rule;
use crate::rorshach::event::Event;
use std::fs::File;
use std::io;

pub struct RuleParser {
    create_rules: Vec<Rule>,
    remove_rules: Vec<Rule>,
    modify_rules: Vec<Rule>
}


impl RuleParser {

    pub fn new() -> Self {
        RuleParser {
            create_rules: Vec::new(),
            remove_rules: Vec::new(),
            modify_rules: Vec::new(),
        }
    }

    pub fn parse_rules(&mut self, config_path: &str) -> Result<(), io::Error> {
        let file = File::open(config_path)?;
        let mut csv_file = csv::ReaderBuilder::new().has_headers(false).delimiter(b'\t').from_reader(file);

        for record in csv_file.deserialize() {
            let rule: Rule = record?;
            match rule.get_event() {
                Event::CREATE => self.create_rules.push(rule),
                Event::MODIFY => self.modify_rules.push(rule),
                Event::DELETE => self.remove_rules.push(rule),
            }
        }

        Ok(())
    }

    pub fn get_create_rules(&self) -> &Vec<Rule> {
        &self.create_rules
    }

    pub fn get_modify_rules(&self) -> &Vec<Rule> {
        &self.modify_rules
    }

    pub fn get_remove_rules(&self) -> &Vec<Rule> {
        &self.remove_rules
    }
}

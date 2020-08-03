
use crate::rorshach::rule::Rule;
use std::fs::File;
use std::io;

pub struct RuleParser {
    rules: Vec<Rule>,
}

impl RuleParser {

    pub fn new() -> Self {
        RuleParser {
            rules: Vec::new(),
        }
    }

    pub fn parse_rules(&mut self, config_path: &str) -> Result<(), io::Error> {
        let file = File::open(config_path)?;
        let mut csv_file = csv::ReaderBuilder::new().has_headers(false).delimiter(b'\t').from_reader(file);

        for record in csv_file.deserialize() {
            let rule: Rule = record?;
            self.rules.push(rule);
        }

        Ok(())
    }

    pub fn get_rules(&self) -> &Vec<Rule> {
        &self.rules
    }

}

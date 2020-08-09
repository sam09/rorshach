
use crate::rorshach::rule::Rule;
use std::fs::File;
use anyhow::{Result, Context};

pub struct RuleParser {
    rules: Vec<Rule>,
}

impl RuleParser {

    pub fn new() -> Self {
        RuleParser {
            rules: Vec::new(),
        }
    }

    pub fn parse_rules(&mut self, config_path: &str) -> Result<()> {
        let file = File::open(config_path).with_context(|| format!("No such file or directory {}", &config_path))?;
        let mut csv_file = csv::ReaderBuilder::new().has_headers(false).delimiter(b'\t').from_reader(file);

        for record in csv_file.deserialize() {
            let rule: Rule = record.with_context(|| format!("Invalid file syntax in config file"))?;
            self.rules.push(rule);
        }

        Ok(())
    }

    pub fn get_rules(&self) -> &Vec<Rule> {
        &self.rules
    }

}

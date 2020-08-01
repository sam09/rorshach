use std::process::Command;
use crate::rorshach::rule::Rule;
use crate::rorshach::rule_parser::RuleParser;
use std::path::PathBuf;
use hotwatch::Event as FileEvent;
use regex::Regex;

pub struct Executor {
    dir: String,
    rules: RuleParser
}

impl Executor {
    pub fn new(dir: String, rules: RuleParser) -> Self {
        Executor{dir: dir, rules: rules}
    }

    fn exec_rule(&self, path: &PathBuf, rule: &Rule) -> Result < (), regex::Error> {
        let path_str = path.to_string_lossy().to_string();
        let re_str = format!("^{}$", rule.get_file_pattern());
        let re = Regex::new(&re_str)?;
        if re.is_match(&path_str) {
            Command::new("sh")
                .arg("-c")
                .arg(rule.get_cmd())
                .env("FULLPATH", &path_str)
                .env("BASEDIR", &self.dir)
                .spawn()
                .expect("Failed to run command");
        }
        Ok(())
    }

    fn filter_and_exec_rules(&self, path: &PathBuf, rules: &Vec<Rule>) {
        for rule in rules {
            if let Err(e) = self.exec_rule(path, rule) {
                println!("Failed to execute {} on file {:?}. Error {:?}", rule.get_cmd(), &path, e);
            }
        }
    }

    pub fn run(&self, event: &FileEvent) {
        match event {
            FileEvent::Create(path) => {
                println!("File {} created", path.display());
                self.filter_and_exec_rules(&path, &self.rules.get_create_rules());
            },
            FileEvent::Write(path) => {
                println!("File {} changed", path.display());
                self.filter_and_exec_rules(&path, &self.rules.get_modify_rules());
            },
            FileEvent::Rename(oldpath, newpath) => {
                self.filter_and_exec_rules(&oldpath, &self.rules.get_remove_rules());
                self.filter_and_exec_rules(&newpath, &self.rules.get_create_rules());
                println!("{} renamed to {}", oldpath.display(), newpath.display());
            },
            FileEvent::Remove(path) => {
                println!("File {} deleted", path.display());
                self.filter_and_exec_rules(&path, &self.rules.get_remove_rules());
            },
            _ => (),
        };
    }
}
#[macro_use] extern crate clap;
#[macro_use] extern crate strum_macros;
extern crate csv;
extern crate shellexpand;
use clap::{App};
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use hotwatch::{
    blocking::{Flow, Hotwatch},
    Event as FileEvent};
use regex::Regex;

mod rorshach;
use crate::rorshach::rule::Rule;
use crate::rorshach::rule_parser::RuleParser;


fn exec_rule(path: &PathBuf, rule: &Rule, dir: &str) -> Result < (), regex::Error> {
    let path_str = path.to_string_lossy().to_string();
    let re_str = format!("^{}$", rule.get_file_pattern());
    let re = Regex::new(&re_str)?;
    if re.is_match(&path_str) {
        Command::new("sh")
            .arg("-c")
            .arg(rule.get_cmd())
            .env("FULLPATH", &path_str)
            .env("BASEDIR", &dir)
            .spawn()
            .expect("Failed to run command");
    }
    Ok(())
}

fn filter_and_exec_rules(path: &PathBuf, rules: &Vec<Rule>, dir: &str) {
    for rule in rules {
        if let Err(e) = exec_rule(path, rule, &dir) {
            println!("Failed to execute {} on file {:?}. Error {:?}", rule.get_cmd(), &path, e);
        }
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let default_config = &shellexpand::tilde("~/.rorshach.conf");
    let config = matches.value_of("config").unwrap_or(default_config);
    let time = matches.value_of("time").unwrap_or("1").parse::<u64>().unwrap();
    let dir = matches.value_of("file").unwrap();
    let duration = Duration::new(time, 0);
    let mut rules = RuleParser::new();
    rules.parse_rules(config).expect("error reading config file");
    let dir_string = dir.to_string();
    let mut hotwatch = Hotwatch::new_with_custom_delay(duration).expect("Error occured created watcher");
    hotwatch.watch(&dir, move |event| {
        match event {
            FileEvent::Create(path) => {
                println!("File {} created", path.display());
                filter_and_exec_rules(&path, rules.get_create_rules(), &dir_string);
            },
            FileEvent::Write(path) => {
                println!("File {} changed", path.display());
                filter_and_exec_rules(&path, rules.get_modify_rules(), &dir_string);
            },
            FileEvent::Rename(oldpath, newpath) => {
                filter_and_exec_rules(&oldpath, rules.get_remove_rules(), &dir_string);
                filter_and_exec_rules(&newpath, rules.get_create_rules(), &dir_string);
                println!("{} renamed to {}", oldpath.display(), newpath.display());
            },
            FileEvent::Remove(path) => {
                println!("File {} deleted", path.display());
                filter_and_exec_rules(&path, rules.get_remove_rules(), &dir_string);
            },
            _ => (),
        };
        Flow::Continue
    }).expect("Error initialising file");

    hotwatch.run();
}

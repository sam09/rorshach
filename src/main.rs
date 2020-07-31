#[macro_use] extern crate clap;
#[macro_use] extern crate strum_macros;
extern crate csv;
extern crate shellexpand;
use clap::{App};
use std::fs::File;
use std::io::{self};
use std::fmt;
use std::path::PathBuf;
use std::process::Command;
use serde::Deserialize;
use std::time::Duration;
extern crate strum;
use hotwatch::{
    blocking::{Flow, Hotwatch},
    Event as FileEvent};
use regex::Regex;


#[derive(Debug, Deserialize, EnumString, Display)]
enum Event {
    CREATE,
    MODIFY,
    DELETE
}

#[derive(Debug, Deserialize)]
struct Rule {
    event: Event,
    file_pattern: String,
    cmd: String,
}


impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.event, self.file_pattern, self.cmd)
    }
}

fn parse_rules(config_path: &str) -> Result<(Vec<Rule>, Vec<Rule>, Vec<Rule>), io::Error> {
    let file = File::open(config_path)?;
    let mut csv_file = csv::ReaderBuilder::new().has_headers(false).delimiter(b'\t').from_reader(file);

    let mut create_rules: Vec<Rule> = Vec::new();
    let mut remove_rules: Vec<Rule> = Vec::new();
    let mut modify_rules: Vec<Rule> = Vec::new();

    for record in csv_file.deserialize() {
        let rule: Rule = record?;
        match rule.event {
            Event::CREATE => create_rules.push(rule),
            Event::MODIFY => modify_rules.push(rule),
            Event::DELETE => remove_rules.push(rule),
        }
    }

    Ok((create_rules, remove_rules, modify_rules))
}


fn exec_rule(path: &PathBuf, rule: &Rule) -> Result < (), regex::Error> {
    let path_str = path.to_string_lossy().to_string();
    let re = Regex::new(&regex::escape(&rule.file_pattern))?;
    if re.is_match(&rule.file_pattern) {
        Command::new("sh")
        .arg("-c")
        .arg(&rule.cmd)
        .env("FULLPATH", &path_str)
        .spawn()
        .expect("Failed to run command");
    }
    Ok(())
}

fn filter_and_exec_rules(path: &PathBuf, rules: &Vec<Rule> ) {
    for rule in rules {
        if let Err(e) = exec_rule(path, rule) {
            println!("Failed to execute {} on file {:?}. Error {:?}", &rule.cmd, &path, e);
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
    let (create_rules, remove_rules, modify_rules) = parse_rules(config).expect("error reading config file");

    let mut hotwatch = Hotwatch::new_with_custom_delay(duration).expect("Error occured created watcher");
    hotwatch.watch(dir, move |event: FileEvent| {
        match event {
            FileEvent::Create(path) => {
                filter_and_exec_rules(&path, &create_rules);
            },
            FileEvent::Write(path) => {
                filter_and_exec_rules(&path, &modify_rules);
            },
            FileEvent::Rename(oldpath, newpath) => {
                filter_and_exec_rules(&oldpath, &remove_rules);
                filter_and_exec_rules(&newpath, &create_rules);
                println!("{} renamed to {}", oldpath.display(), newpath.display());
            },
            FileEvent::Remove(path) => {
                filter_and_exec_rules(&path, &remove_rules);
            },
            FileEvent::NoticeWrite(path) => {
                println!("File {:?} changed", path);
            },
            FileEvent::NoticeRemove(path) => {
                println!("File {:?} deleted", path);
            },
            _ => {
                println!("Discarding events not tracked by rorshach!");
            }
        };
        Flow::Continue
    }).expect("Error initialising file");

    hotwatch.run();
}

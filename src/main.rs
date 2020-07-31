#[macro_use]
extern crate clap;
extern crate csv;
use clap::{App};
use std::fs::File;
use std::path::Path;
use std::io::{self};
use std::fmt;
use serde::Deserialize;
use std::time::Duration;
extern crate strum;
use hotwatch::{
    blocking::{Flow, Hotwatch},
    Event as FileEvent};
#[macro_use] extern crate strum_macros;


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

fn parse_rules(config_path: String) -> Result<Vec<Rule>, io::Error> {
    let file = File::open(&config_path)?;
    let mut csv_file = csv::ReaderBuilder::new().has_headers(false).delimiter(b'\t').from_reader(file);

    let mut rules: Vec<Rule> = Vec::new();
    for record in csv_file.deserialize() {
        let rule: Rule = record?;
        rules.push(rule);
    }
    Ok(rules)
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let config = matches.value_of("config").unwrap_or("/home/sam/default.conf");
    let time = matches.value_of("time").unwrap_or("1").parse::<u64>().unwrap();
    let dir = matches.value_of("file").unwrap();
    let duration = Duration::new(time, 0);
    let path = Path::new(dir);
    let rules = parse_rules(config.to_string()).expect("error reading config file");

    let mut hotwatch = Hotwatch::new_with_custom_delay(duration).expect("Error occured created watcher");
    hotwatch.watch(dir, move |event: FileEvent| {
        match event {
            FileEvent::Create(path) => {
                println!("File {:?} created some time ago", path);
            },
            FileEvent::Write(path) => {
                println!("File {:?} changed some time ago", path);
            },
            FileEvent::Rename(oldpath, newpath) => {
                println!("File renamed {:?} to {:?}", oldpath, newpath);
            },
            FileEvent::Remove(path) => {
                println!("File {:?} deleted some time ago", path);
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

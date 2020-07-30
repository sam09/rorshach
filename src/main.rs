#[macro_use]
extern crate clap;
extern crate csv;
use clap::{App};
use std::fs::File;
use std::io::{self};
use std::fmt;
use serde::Deserialize;
extern crate strum;
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


fn watch_files(rules: Vec<Rule>, time_in_seconds: u64, dir: String) -> Result<(), io::Error> {
    unimplemented!();
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let config = matches.value_of("config").unwrap_or("/home/sam/default.conf");
    let time = matches.value_of("time").unwrap_or("5").parse::<u64>().unwrap();
    let dir = matches.value_of("file").unwrap();

    println!("Config: {}, File {}, Time {}", config, dir, time);


    let rules = parse_rules(config.to_string()).expect("error reading config file");
    watch_files(rules, time, dir.to_string());
}

#[macro_use] extern crate clap;
#[macro_use] extern crate strum_macros;
extern crate csv;
extern crate shellexpand;
use clap::{App};
use std::time::Duration;
use hotwatch::blocking::{Flow, Hotwatch};

mod rorshach;
use crate::rorshach::rule_parser::RuleParser;
use crate::rorshach::executor::Executor;


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
    let executor = Executor::new(dir_string, rules);
    hotwatch.watch(&dir, move |event| {
        executor.run(&event);
        Flow::Continue
    }).expect("Error initialising file");

    hotwatch.run();
}

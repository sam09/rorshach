#[macro_use] extern crate clap;
#[macro_use] extern crate strum_macros;
extern crate csv;
extern crate shellexpand;
use clap::{App};
use std::time::Duration;
use hotwatch::blocking::{Flow, Hotwatch};

extern crate log;
extern crate simple_logger;
use log::error;

mod rorshach;
use crate::rorshach::rule_parser::RuleParser;
use crate::rorshach::executor::Executor;


fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let default_config = &shellexpand::tilde("~/.rorshach.conf");
    let config = matches.value_of("config").unwrap_or(default_config);
    let time = matches.value_of("time").unwrap_or("1").parse::<u64>().unwrap();
    let dir = matches.value_of("file").unwrap();
    let duration = Duration::new(time, 0);
    let mut rules = RuleParser::new();
    match rules.parse_rules(config) {
        Err(e) => {
            error!("Error occurred parsing rules {}", e);
            std::process::exit(1);
        },
        _ => (),

    }
    let dir_string = dir.to_string();
    let mut hotwatch = match Hotwatch::new_with_custom_delay(duration) {
        Err(e) => {
            error!("Error occured created watcher {}", e);
            std::process::exit(1);
        },
        Ok(v) => {
            v
        }
    };

    let executor = Executor::new(dir_string, rules);
    match hotwatch.watch(&dir, move |event| {
        executor.run(&event);
        Flow::Continue
    }) {
        Err(e) => {
            error!("Error initalising file watcher for {}: {}", dir, e);
            std::process::exit(1);
        },
        _ => (),
    }

    hotwatch.run();
}

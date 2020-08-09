#[macro_use] extern crate clap;
#[macro_use] extern crate strum_macros;
extern crate csv;
extern crate shellexpand;
use clap::{App};
use std::time::Duration;
use hotwatch::blocking::{Flow, Hotwatch};
use anyhow::{Context, Result};

extern crate log;
extern crate simple_logger;
use log::error;

mod rorshach;
use crate::rorshach::rule_parser::RuleParser;
use crate::rorshach::executor::Executor;

fn main() -> Result<()> {
    simple_logger::init_with_level(log::Level::Info).context("Failed to initliaze logger")?;
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let default_config = &shellexpand::tilde("~/.rorshach.conf");
    let config = matches.value_of("config").unwrap_or(default_config);
    let time = matches.value_of("time").unwrap_or("1").parse::<u64>().context("failed to parse Time variable from command line")?;
    let dir = matches.value_of("file").context("No file specified")?;
    let duration = Duration::new(time, 0);
    let mut rules = RuleParser::new();
    rules.parse_rules(config).context("Error occurred parsing rules")?;

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

    hotwatch.watch(&dir, move |event| {
        executor.run(&event);
        Flow::Continue
    }).context("Failed to initalize watcher")?;

    hotwatch.run();
    Ok(())
}

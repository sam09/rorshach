use crate::rorshach::event::Event;
use crate::rorshach::event_type::EventType;
use crate::rorshach::rule_parser::RuleParser;
use crate::rorshach::producer::Producer;
use crate::rorshach::consumer::Consumer;
use hotwatch::Event as FileEvent;
extern crate log;
use log::info;
use crossbeam::thread;
extern crate pub_sub;

pub struct Executor {
    producer: Producer,
    consumers: Vec<Consumer>,
}

impl Executor {

    pub fn new(dir: String, rules: RuleParser) -> Self {
        let channel = pub_sub::PubSub::new();
        let producer = Producer::new(channel.clone());
        let mut consumers = Vec::<Consumer>::new();
        for rule in rules.get_rules() {
            consumers.push(Consumer::new(channel.subscribe().clone(), rule, dir.clone()))
        }
        Executor{producer: producer, consumers: consumers}
    }

    pub fn run(&self, file_event: &FileEvent) {
        match file_event {
            FileEvent::Create(path) => {
                info!("File {} created", path.display());
                self.producer.send(Event::new(EventType::CREATE, path.to_path_buf()));
            },
            FileEvent::Write(path) => {
                info!("File {} changed", path.display());
                self.producer.send(Event::new(EventType::MODIFY, path.to_path_buf()));
            },
            FileEvent::Rename(old_path, new_path) => {
                self.producer.send(Event::new(EventType::DELETE, old_path.to_path_buf()));
                self.producer.send(Event::new(EventType::MODIFY, new_path.to_path_buf()));
                info!("{} renamed to {}", old_path.display(), new_path.display());
            },
            FileEvent::Remove(path) => {
                self.producer.send(Event::new(EventType::DELETE, path.to_path_buf()));
                info!("File {} deleted", path.display());
            },
            _ => return,
        };
        self.start_consumers();
    }

    pub fn start_consumers(&self) {
        for consumer in &self.consumers {
            consumer.consume();
        }
    }
}
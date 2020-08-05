use crate::rorshach::event::Event;
use crate::rorshach::event_type::EventType;
use crate::rorshach::rule_parser::RuleParser;
use crate::rorshach::producer::Producer;
use crate::rorshach::consumer::Consumer;
use hotwatch::Event as FileEvent;
extern crate log;
use log::info;
extern crate pub_sub;
use futures::{ join, executor::block_on};

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

    async fn produce(&self, file_event: &FileEvent) {
        match file_event {
            FileEvent::Create(path) => {
                info!("File {} created", path.display());
                self.producer.send(Event::new(EventType::CREATE, Some(path.to_path_buf()), None));
            },
            FileEvent::Write(path) => {
                info!("File {} changed", path.display());
                self.producer.send(Event::new(EventType::MODIFY, Some(path.to_path_buf()), None));
            },
            FileEvent::Rename(old_path, new_path) => {
                self.producer.send(
                    Event::new(EventType::RENAME, Some(old_path.to_path_buf()),Some(new_path.to_path_buf()))
                );
                info!("{} renamed to {}", old_path.display(), new_path.display());
            },
            FileEvent::Remove(path) => {
                self.producer.send(Event::new(EventType::DELETE, Some(path.to_path_buf()), None));
                info!("File {} deleted", path.display());
            },
            _ => self.producer.send(Event::new(EventType::UNSUPPORTED, None, None)),
        };
    }

    async fn start_consumers(&self) {
        for consumer in &self.consumers {
            consumer.consume().await;
        }
    }

    async fn async_run(&self, file_event: &FileEvent) {
        join!(self.produce(file_event), self.start_consumers());
    }

    pub fn run(&self, file_event: &FileEvent) {
        block_on(self.async_run(file_event))
    }
}
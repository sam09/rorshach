use pub_sub::PubSub;
use crate::rorshach::event::Event;
use log::{error, debug};

pub struct Producer {
    sender: PubSub<Event>
}

impl Producer {

    pub fn new(sender: PubSub<Event> ) -> Self {
        Producer{ sender}
    }

    pub fn send(&self, event: Event) {
        match self.sender.send(event) {
            Err(err) => {
                error!("Error occurred sending event: {}", err);
            },
            Ok(_) => debug!("Successfully sent event")
        };
    }

}
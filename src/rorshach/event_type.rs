extern crate strum;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize, Copy, Clone, EnumString, Display)]
pub enum EventType {
    CREATE,
    MODIFY,
    DELETE,
}

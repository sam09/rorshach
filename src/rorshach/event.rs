extern crate strum;
use serde::Deserialize;

#[derive(Debug, Deserialize, Copy, Clone, EnumString, Display)]
pub enum Event {
    CREATE,
    MODIFY,
    DELETE,
}

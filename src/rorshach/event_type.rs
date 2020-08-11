use strum_macros::{EnumString, Display};
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize, Copy, Clone, EnumString, Display)]
pub enum EventType {
    CREATE,
    MODIFY,
    DELETE,
    RENAME,
    UNSUPPORTED,
}

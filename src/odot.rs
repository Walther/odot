use crate::message::*;
use crate::tags::*;
use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Odot {
    message: Message,
    tags: Tags,
    timestamp: DateTime<Utc>,
    uuid: Uuid,
}

impl Odot {
    pub fn new(message: Message, tags: Tags) -> Odot {
        Odot {
            message,
            tags,
            timestamp: Utc::now(),
            uuid: Uuid::new_v4(),
        }
    }

    pub fn get_uuid(self) -> Uuid {
        self.uuid
    }

    pub fn get_message(self) -> Message {
        self.message
    }
}

#[test]
fn get_message() {
    let odot = Odot::new("Test get_message".to_string(), Tags::new());
    assert_eq!("Test get_message", odot.get_message());
}

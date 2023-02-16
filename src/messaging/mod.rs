use async_trait::async_trait;
use std::{collections::HashMap, error::Error};
mod messages;
mod sqs_messanger;
pub use messages::*;
pub use sqs_messanger::SqsMessanger;

#[async_trait]
pub trait Messanger: Sync + Send {
    type Message: Message;
    async fn send_message(&self, message: Self::Message) -> Result<(), Box<dyn Error>>;
}

pub trait Message: Send + Sync {
    const QUEUE_NAME: &'static str;
    fn body(self) -> String;
    fn attributes(&mut self) -> Option<HashMap<String, String>>;
}

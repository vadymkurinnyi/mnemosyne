use super::Message;
use std::collections::HashMap;

pub struct UserMessage {
    body: String,
    attributes: Option<HashMap<String, String>>,
}
impl UserMessage {
    pub fn new(body: impl Into<String>) -> Self {
        Self {
            body: body.into(),
            attributes: Default::default(),
        }
    }
}
impl Message for UserMessage {
    const QUEUE_NAME: &'static str = "Customers";
    fn body(self) -> String {
        self.body
    }

    fn attributes(&mut self) -> Option<HashMap<String, String>> {
        self.attributes.take()
    }
}

use anyhow::anyhow;
use async_trait::async_trait;
use aws_config::SdkConfig;
use aws_sdk_sqs::{client::fluent_builders::SendMessage, model::MessageAttributeValue, Client};
use std::{collections::HashMap, error::Error};
use tokio::sync::Mutex;

use super::{messages::UserMessage, Message, Messanger};

pub struct SqsMessanger {
    client: Client,
    queue_url_map: Mutex<HashMap<String, String>>,
}
impl SqsMessanger {
    pub async fn new(config: &SdkConfig) -> Result<Self, anyhow::Error> {
        let client = Client::new(&config);
        Ok(Self {
            client,
            queue_url_map: Mutex::new(Default::default()),
        })
    }
    async fn get_queue_url(&self, name: &str) -> Result<String, Box<dyn Error>> {
        if let Some(name) = self.queue_url_map.lock().await.get(name) {
            return Ok(name.to_string());
        }

        let queue = self.client.get_queue_url().queue_name(name).send().await?;
        let queue_url = queue
            .queue_url()
            .ok_or(anyhow!(format!("queue_url not found for {name}")))?;
        self.queue_url_map
            .lock()
            .await
            .insert(name.to_string(), queue_url.to_string());
        Ok(queue_url.to_owned())
    }
    fn set_attributes(
        message: SendMessage,
        attributes: Option<HashMap<String, String>>,
    ) -> SendMessage {
        if let Some(attributes) = attributes {
            if attributes.is_empty() {
                return message;
            }
            let map = attributes
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        MessageAttributeValue::builder()
                            .data_type("String")
                            .string_value(v)
                            .build(),
                    )
                })
                .collect::<HashMap<String, MessageAttributeValue>>();
            return message.set_message_attributes(Some(map));
        }
        message
    }
}

#[async_trait]
impl Messanger for SqsMessanger {
    type Message = UserMessage;
    async fn send_message(&self, message: Self::Message) -> Result<(), Box<dyn Error>> {
        let mut message = message;
        let attributes = message.attributes().take();
        let url = self.get_queue_url(Self::Message::QUEUE_NAME).await?;
        let send = self
            .client
            .send_message()
            .queue_url(url)
            .message_body(message.body());
        let send = SqsMessanger::set_attributes(send, attributes);
        send.send().await?;
        Ok(())
    }
}

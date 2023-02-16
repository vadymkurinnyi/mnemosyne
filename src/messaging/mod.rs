use anyhow::anyhow;
use async_trait::async_trait;
use aws_config::SdkConfig;
use aws_sdk_sqs::Client;
use std::error::Error;

#[async_trait]
pub trait Messanger: Sync + Send {
    async fn send_message(&self, message: &str) -> Result<(), Box<dyn Error>>;
}

pub struct SqsMessanger {
    queue_url: String,
    client: Client,
}

impl SqsMessanger {
    pub async fn new(config: &SdkConfig) -> Result<Self, anyhow::Error> {
        let client = Client::new(&config);
        let queue = client
            .get_queue_url()
            .queue_name("Customers")
            .send()
            .await?;
        let queue_url = queue
            .queue_url()
            .ok_or(anyhow!("queue_url not found for Customers"))?
            .to_string();
        Ok(Self { queue_url, client })
    }
}

#[async_trait]
impl Messanger for SqsMessanger {
    async fn send_message(&self, message: &str) -> Result<(), Box<dyn Error>> {
        self.client
            .send_message()
            .queue_url(&self.queue_url)
            .message_body(message)
            .send()
            .await?;
        Ok(())
    }
}

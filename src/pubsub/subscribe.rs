use futures_lite::stream::StreamExt;
use lapin::options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions};
use lapin::types::FieldTable;
use lapin::Channel;
use serde::Deserialize;
use std::error::Error;

pub enum AckType {
    Ack,
    NackRequeue,
    NackDiscard,
}

pub async fn subscribe_json<T, F>(
    channel: &Channel,
    queue_name: &str,
    mut handler: F,
) -> Result<(), Box<dyn Error>>
where
    T: for<'de> Deserialize<'de>,
    F: FnMut(T) -> AckType,
{
    let consumer_tag = String::new();

    let mut consumer = channel
        .basic_consume(
            queue_name,
            consumer_tag.as_str(),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!(
        " [*] Waiting for messages on {}. To exit press CTRL+C",
        queue_name
    );

    while let Some(delivery_result) = consumer.next().await {
        let delivery = delivery_result?;
        let value = serde_json::from_slice::<serde_json::Value>(&delivery.data)?;
        match serde_json::from_value::<T>(value.clone()) {
            Ok(target) => match handler(target) {
                AckType::Ack => {
                    delivery.ack(BasicAckOptions::default()).await?;
                    println!("Acked");
                }
                AckType::NackRequeue => {
                    delivery
                        .nack(BasicNackOptions {
                            multiple: false,
                            requeue: true,
                        })
                        .await?;
                    println!("Nack Requeue");
                }
                _ => {
                    delivery
                        .nack(BasicNackOptions {
                            multiple: false,
                            requeue: false,
                        })
                        .await?;
                    println!("Nack Discarded");
                }
            },
            Err(e) => {
                eprintln!(
                    "Error deserializing message: {:?}, raw message: {}",
                    e, value
                );
                delivery.nack(BasicNackOptions::default()).await?;
            }
        }
    }

    Ok(())
}

use std::error::Error;
use lapin::{Channel, Queue};
use lapin::options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions};
use lapin::types::FieldTable;
use serde::Deserialize;
use futures_lite::stream::StreamExt;

pub async fn subscribe_json<T, F>(
    channel: &Channel,
    queue: &Queue,
    mut handler: F,
) -> Result<(), Box<dyn Error>>
where
    T: for<'de> Deserialize<'de>,
    F: FnMut(T),
{
    let consumer_tag = String::new();

    let mut consumer = channel
        .basic_consume(
            queue.name().as_str(),
            consumer_tag.as_str(),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!(" [*] Waiting for messages on {}. To exit press CTRL+C", queue.name());
    while let Some(delivery_result) = consumer.next().await {
        let delivery = delivery_result?;
        let value = serde_json::from_slice::<serde_json::Value>(&delivery.data)?;
        match serde_json::from_value::<T>(value.clone()) {
            Ok(target) => {
                handler(target);
                delivery.ack(BasicAckOptions::default()).await?;
            }
            Err(e) => {
                eprintln!("Error deserializing message: {:?}, raw message: {}", e, value);
                delivery.nack(BasicNackOptions::default()).await?;
            }
        }
    }

    Ok(())
}
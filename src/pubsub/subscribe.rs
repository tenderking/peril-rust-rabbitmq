use futures_lite::stream::StreamExt;
use lapin::message::Delivery;
use lapin::options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions, BasicQosOptions};
use lapin::types::FieldTable;
use lapin::Channel;
use postcard::from_bytes;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;

pub enum AckType {
    Ack,
    NackRequeue,
    NackDiscard,
}

async fn subscribe<T, F, D>(
    channel: &Channel,
    queue_name: &str,
    mut handler: F,
    deserializer: D,
) -> Result<(), Box<dyn Error + Send + Sync>>
// Updated
where
    T: for<'de> Deserialize<'de>,
    F: FnMut(T) -> AckType,
    D: Fn(&Delivery) -> Result<T, Box<dyn Error + Send + Sync>>, // Updated
{
    let _qos = channel.basic_qos(10, BasicQosOptions::default());
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
        match deserializer(&delivery) {
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
                eprintln!("Error deserializing message: {:?}, ", e);
                delivery.nack(BasicNackOptions::default()).await?;
            }
        }
    }

    Ok(())
}

fn json_deserialize<T: for<'de> Deserialize<'de>>(
    delivery: &Delivery,
) -> Result<T, Box<dyn Error + Send + Sync>> {
    let value = serde_json::from_slice::<Value>(&delivery.data)?;
    let unmarshalled = serde_json::from_value::<T>(value.clone())?;
    Ok(unmarshalled)
}
fn postcard_deserialize<'a, T: Deserialize<'a>>(
    delivery: &'a Delivery,
) -> Result<T, Box<dyn Error + Send + Sync>> {
    let unmarshalled: T = from_bytes(&delivery.data)?;
    Ok(unmarshalled)
}
pub async fn subscribe_json<T, F>(
    channel: &Channel,
    queue_name: &str,
    handler: F,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    T: for<'de> Deserialize<'de>,
    F: FnMut(T) -> AckType,
{
    subscribe(channel, queue_name, handler, |delivery| {
        json_deserialize::<T>(delivery)
    })
    .await
}

pub async fn subscribe_postcard<T, F>(
    channel: &Channel,
    queue_name: &str,
    handler: F,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    T: for<'de> Deserialize<'de>,
    F: FnMut(T) -> AckType,
{
    subscribe(channel, queue_name, handler, |delivery| {
        postcard_deserialize::<T>(delivery)
    })
    .await
}

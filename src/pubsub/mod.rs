use lapin::options::{
    BasicPublishOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions,
};
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel, Connection, Error, Queue};
use serde::Serialize;
use std::time::Duration;
use tokio::time::timeout;
use crate::routing::Exchange;

pub async fn publish_json<T: Serialize>(
    ch: Channel,
    exchange:  Exchange,
    key: &str,
    val: T,
) -> Result<(), Error> {
    let body = serde_json::to_string(&val).unwrap();
    match ch
        .exchange_declare(
            exchange.as_str(), // Exchange name
            exchange.exchange_type(),
            ExchangeDeclareOptions {
                passive: false,     // Don't check if the exchange exists
                durable: true,      // Make the exchange persistent
                auto_delete: false, // Don't delete the exchange when unused
                internal: false,    // Allow external publishers/consumers
                nowait: false,      // Wait for a confirmation from the server
            },
            FieldTable::default(), // No special arguments
        )
        .await
    {
        Ok(_) => {}
        Err(_e_) => {}
    }
    match timeout(
        Duration::from_secs(5), // 5-second timeout
        ch.basic_publish(
            exchange.as_str(),
            &key,
            BasicPublishOptions::default(),
            body.as_bytes(),
            BasicProperties::default().with_content_type("application/json".into()),
        ),
    )
    .await
    {
        Ok(_) => {}   // Propagate the Result from basic_publish
        Err(_e_) => {} // Return a timeout error
    };

    println!("Message published!");
    Ok(())
}
pub enum SimpleQueueType {
    Transient,
    Durable,
}

impl SimpleQueueType {
    pub fn as_bool(&self) -> bool {
        match self {
            SimpleQueueType::Transient => true,
            SimpleQueueType::Durable => false,
        }
    }
}


pub async fn declare_and_bind(
    conn: Connection,
    exchange: &str,
    key: &str,
    simple_queue_type: SimpleQueueType,
) -> Result<(Channel, Queue), Error> {
    let rabbitmq_channel = match conn.create_channel().await {
        Ok(channel) => channel,
        Err(err) => {
            eprintln!("Error creating RabbitMQ channel: {}", err);
            return Err(err.into());
        }
    };

    let q: Queue = match rabbitmq_channel
        .queue_declare(
            exchange,
            QueueDeclareOptions {
                nowait: false,
                auto_delete: !simple_queue_type.as_bool(),
                durable: !simple_queue_type.as_bool(),
                passive: false,
                exclusive: !simple_queue_type.as_bool(),
            },
            FieldTable::default(),
        )
        .await
    {
        Ok(queue) => queue,
        Err(err) => {
            eprintln!("Error creating RabbitMQ queue: {}", err);
            return Err(err.into());
        }
    };

    match rabbitmq_channel
        .queue_bind(
            &q.name().as_str(),
            exchange,
            key,
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await
    {
        Ok(bind) => bind,
        Err(err) => {
            eprintln!("Error creating RabbitMQ queue: {}", err);
            return Err(err.into());
        }
    }

    Ok((rabbitmq_channel, q))
}

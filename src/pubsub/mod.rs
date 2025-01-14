use lapin::{Channel, Error, Queue};
use lapin::options::{QueueBindOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use crate::routing::RoutingKey;

pub mod publish;
pub mod subscribe;

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
    ch: &Channel,
    exchange: &str,
    key: RoutingKey,
    simple_queue_type: SimpleQueueType,
) -> Result<Queue, Error> {


    let q: Queue = match ch
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

    match ch
        .queue_bind(
            &q.name().as_str(),
            exchange,
            &*key.as_str(),
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

    Ok( q)
}

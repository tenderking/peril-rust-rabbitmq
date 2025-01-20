use lapin::options::{QueueBindOptions, QueueDeclareOptions};
use lapin::types::{AMQPValue, FieldTable};
use lapin::{Channel, Error, Queue};

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
    q_name: &str,
    key: &str,
    simple_queue_type: &SimpleQueueType,
) -> Result<Queue, Error> {
    let mut queue_args = FieldTable::default();
    queue_args.insert(
        "x-dead-letter-exchange".into(),
        AMQPValue::LongString("peril_dlx".into()),
    );
    queue_args.insert(
        "x-dead-letter-routing-key".into(),
        AMQPValue::LongString("dlq.failed".into()), // Routing key for DLQ
    );
    let q: Queue = match ch
        .queue_declare(
            q_name,
            QueueDeclareOptions {
                nowait: false,
                auto_delete: !simple_queue_type.as_bool(),
                durable: !simple_queue_type.as_bool(),
                passive: false,
                exclusive: !simple_queue_type.as_bool(),
            },
            queue_args,
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
            &q_name,
            &exchange,
            &key,
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

    Ok(q)
}

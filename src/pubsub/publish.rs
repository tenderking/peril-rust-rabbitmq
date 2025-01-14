

use lapin::options::{
    BasicPublishOptions,
};
use lapin::{BasicProperties, Channel, Error};
use serde::Serialize;
use std::time::Duration;
use tokio::time::timeout;
use crate::routing::{Exchange};

pub async fn publish_json<T: Serialize>(
    ch: Channel,
    exchange:  Exchange,
    key: &str,
    val: T,
) -> Result<(), Error> {
    let body = serde_json::to_string(&val).unwrap();

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
        Ok(_) => {}
        Err(_e_) => {}
    };

    println!("Message published!");
    Ok(())
}



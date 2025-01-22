use crate::routing::Exchange;
use lapin::options::BasicPublishOptions;
use lapin::types::ShortString;
use lapin::{BasicProperties, Channel, Error};
use postcard::to_allocvec;
use serde::Serialize;
use std::time::Duration;
use tokio::time::timeout;

pub async fn publish_json<T: Serialize>(
    ch: Channel,
    exchange: Exchange,
    key: &str,
    val: T,
) -> Result<(), Error> {
    publish(ch, exchange, key, val, |val: T| json_serializer::<T>(&val)).await
}
pub async fn publish_postcard<T: Serialize>(
    ch: Channel,
    exchange: Exchange,
    key: &str,
    val: T,
) -> Result<(), Error> {
    publish(ch, exchange, key, val, |val: T| {
        postcard_serializer::<T>(val)
    })
    .await
}
async fn publish<T: Serialize, D: Fn(T) -> (String, ShortString)>(
    ch: Channel,
    exchange: Exchange,
    key: &str,
    val: T,
    serializer: D,
) -> Result<(), Error> {
    let (body, string) = serializer(val);

    match timeout(
        Duration::from_secs(5), // 5-second timeout
        ch.basic_publish(
            exchange.as_str(),
            &key,
            BasicPublishOptions::default(),
            body.as_bytes(),
            BasicProperties::default().with_content_type(string),
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

fn json_serializer<T: Serialize>(val: &T) -> (String, ShortString) {
    let serialized_data = serde_json::to_string(&val).unwrap();
    (serialized_data, "application/json".into())
}
fn postcard_serializer<T: Serialize>(val: T) -> (String, ShortString) {
    let serialized_data = String::from_utf8(to_allocvec(&val).unwrap());
    match serialized_data {
        Ok(serialized_data) => (serialized_data, "application/octet-stream".into()),
        Err(_) => panic!("couldnt serialize data"),
    }
}

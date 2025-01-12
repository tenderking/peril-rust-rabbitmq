use std::sync::mpsc;
use std::thread;
use signal_hook::{consts::TERM_SIGNALS, iterator::Signals};
use lapin::{Connection, ConnectionProperties, Channel, ExchangeKind, BasicProperties};
use lapin::ExchangeKind::Custom;
use lapin::options::{BasicPublishOptions, ExchangeDeclareOptions};
use lapin::types::FieldTable;
use serde::Serialize;

async fn publish_json<T: Serialize>(ch: Channel, exchange: &str, key:String, val:T ){
    let body = serde_json::to_string(&val).unwrap();
    let exchange_kind:ExchangeKind = ExchangeKind::Custom(String::from(exchange));
   match  ch.exchange_declare(
        "my_exchange", // Exchange name
        exchange_kind,
        ExchangeDeclareOptions {
            passive: false,  // Don't check if the exchange exists
            durable: true,   // Make the exchange persistent
            auto_delete: false, // Don't delete the exchange when unused
            internal: false, // Allow external publishers/consumers
            nowait: false,   // Wait for a confirmation from the server
        },
        FieldTable::default(), // No special arguments
    ).await {
       Ok(_) => {},
       Err(e) => {}
   }
   match  ch.basic_publish(
        exchange,   // Exchange name
        "",        // Routing key
        BasicPublishOptions::default(), // Basic publish options
        &body.as_ref(),
        BasicProperties::default(),       // Message properties
    )
        .await {
       Ok(_) => {},
       Err(e) => {}
   }


}
#[tokio::main]
async fn main() {
    const ADDR: &str = "amqp://guest:guest@localhost:5672/my_vhost";


    let (done_sender, done_receiver) = mpsc::channel();

    let conn = match Connection::connect(
        &ADDR,
        ConnectionProperties::default(),
    ).await {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Error connecting to RabbitMQ: {}", err);
            return;
        }
    };

    let rabbitmq_channel = match conn.create_channel().await {
        Ok(channel) => channel,
        Err(err) => {
            eprintln!("Error creating RabbitMQ channel: {}", err);
            return;
        }
    };

    publish_json(rabbitmq_channel, "Fanout", "test".parse().unwrap(), String::from("Hello, World!")).await;
    println!("Starting Peril server...");


    // Create a thread for signal handling
    let signal_done_sender = done_sender.clone();
    thread::spawn(move || {
        let mut signals = Signals::new(TERM_SIGNALS).expect("Unable to create signal handler");
        for signal in signals.forever() {
            println!("\nReceived signal: {:?}", signal);
            signal_done_sender.send(()).expect("Failed to send done signal");
            break; // Exit after receiving the first signal
        }
    });

    println!("Awaiting signal...");

    // Wait for the signal handler to signal completion
    done_receiver.recv().expect("Failed to receive done signal");

    println!("Exiting...");


}

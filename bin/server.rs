use lapin::options::ExchangeDeclareOptions;
use lapin::types::FieldTable;
use lapin::{Connection, ConnectionProperties};
use risk_rust::gamelogic::gamelogic::get_input;
use risk_rust::pubsub::declare_and_bind;
use risk_rust::pubsub::publish::publish_json;
use risk_rust::{pubsub, routing};
use signal_hook::{consts::TERM_SIGNALS, iterator::Signals};
use std::sync::mpsc;
use std::thread;

#[tokio::main]
async fn main() {
    const ADDR: &str = "amqp://guest:guest@localhost:5672/my_vhost";

    let (done_sender, done_receiver) = mpsc::channel();

    let conn = match Connection::connect(&ADDR, ConnectionProperties::default()).await {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Error connecting to RabbitMQ: {}", err);
            return;
        }
    };
    let publish_channel = match conn.create_channel().await {
        Ok(channel) => channel,
        Err(err) => {
            eprintln!("Error creating RabbitMQ channel: {}", err);
            return;
        }
    };

    let _exchange = match &publish_channel
        .clone()
        .exchange_declare(
            routing::Exchange::PerilTopic.as_str(), // Exchange name
            routing::Exchange::PerilTopic.exchange_type(),
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
    };
    let _q = declare_and_bind(
        &publish_channel,
        routing::Exchange::PerilTopic.as_str(),
        routing::Exchange::PerilTopic.as_str(),
        &*routing::RoutingKey::GameLog(String::from("*")).as_str(),
        &pubsub::SimpleQueueType::Durable,
    )
    .await
    .expect("Error binding the queue");

    println!("Starting Peril server...");

    // Create a thread for signal handling
    let signal_done_sender = done_sender.clone();
    loop {
        let word = get_input();

        if word.len() == 0 {
            continue;
        }

        match word[0].as_str() {
            "pause" => {
                println!("Pausing the game");
                publish_json(
                    publish_channel.clone(),
                    routing::Exchange::PerilTopic,
                    &*routing::RoutingKey::Pause(String::from("")).as_str(),
                    routing::PlayingState { is_paused: true },
                )
                .await
                .expect("TODO: panic message");
            }
            "resume" => {
                println!("Resuming the game");
                publish_json(
                    publish_channel.clone(),
                    routing::Exchange::PerilTopic,
                    &*routing::RoutingKey::Pause(String::from("")).as_str(),
                    routing::PlayingState { is_paused: false },
                )
                .await
                .expect("Error publishing message");
            }
            "quit" => {
                done_sender
                    .send(())
                    .expect("Failed to send done signal from main loop");
                break;
            }
            _ => println!("Invalid command. Please try again."),
        }
    }
    thread::spawn(move || {
        let mut signals = Signals::new(TERM_SIGNALS).expect("Unable to create signal handler");
        for signal in signals.forever() {
            println!("\nReceived signal: {:?}", signal);
            signal_done_sender
                .send(())
                .expect("Error publishing message");
            break;
        }
    });

    // Wait for the signal handler to signal completion
    done_receiver.recv().expect("Failed to receive done signal");

    println!("Exiting...");
}

use lapin::{Connection, ConnectionProperties};
use risk_rust::pubsub::publish_json;
use risk_rust::routing;
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

    let rabbitmq_channel = match conn.create_channel().await {
        Ok(channel) => channel,
        Err(err) => {
            eprintln!("Error creating RabbitMQ channel: {}", err);
            return;
        }
    };

    publish_json(
        rabbitmq_channel,
        routing::Exchange::PerilDirect,
        &*routing::RoutingKey::Pause(String::from("")).as_str(),
        routing::PlayingState { is_paused: true },
    )
    .await
    .expect("TODO: panic message");
    println!("Starting Peril server...");

    // Create a thread for signal handling
    let signal_done_sender = done_sender.clone();
    thread::spawn(move || {
        let mut signals = Signals::new(TERM_SIGNALS).expect("Unable to create signal handler");
        for signal in signals.forever() {
            println!("\nReceived signal: {:?}", signal);
            signal_done_sender
                .send(())
                .expect("Failed to send done signal");
            break; // Exit after receiving the first signal
        }
    });

    println!("Awaiting signal...");

    // Wait for the signal handler to signal completion
    done_receiver.recv().expect("Failed to receive done signal");

    println!("Exiting...");
}

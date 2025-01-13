use risk_rust::gamelogic::gamelogic::client_welcome;

use lapin::{Connection, ConnectionProperties};
use risk_rust::pubsub::{declare_and_bind};
use risk_rust::{pubsub, routing};
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::iterator::Signals;
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
    let username = match client_welcome() {
        Ok(username) => username,
        Err(_err) => {
            return;
        }
    };
    print!("Hello, {:?}", &username);
    let (ch,q)=declare_and_bind(
        conn,
        routing::Exchange::PerilDirect.as_str(),
        &*routing::RoutingKey::Pause(String::from(username)).as_str(),
        pubsub::SimpleQueueType::Durable,
    )
    .await
    .expect("TODO: panic message");

    print!("Connected to channel {:?} and created queue {:?}", &ch, &q);

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

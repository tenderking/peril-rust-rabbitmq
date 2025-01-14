use lapin::{Connection, ConnectionProperties};
use risk_rust::gamelogic::gamelogic::get_input;
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
        rabbitmq_channel.clone(),
        routing::Exchange::PerilDirect,
        &*routing::RoutingKey::Pause(String::from("")).as_str(),
        routing::PlayingState { is_paused: true },
    )
    .await
    .expect("TODO: panic message");
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
                    rabbitmq_channel.clone(),
                    routing::Exchange::PerilDirect,
                    &*routing::RoutingKey::Pause(String::from("")).as_str(),
                    routing::PlayingState { is_paused: true },
                )
                .await
                .expect("TODO: panic message");
            }
            "resume" => {
                println!("Resuming the game");
                publish_json(
                    rabbitmq_channel.clone(),
                    routing::Exchange::PerilDirect,
                    &*routing::RoutingKey::Pause(String::from("")).as_str(),
                    routing::PlayingState { is_paused: false },
                )
                .await
                .expect("Error publishing message");
            }
            "quit" => {
                done_sender.send(()).expect("Failed to send done signal from main loop");
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

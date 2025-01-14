use risk_rust::gamelogic::gamelogic::{client_welcome, get_input, print_client_help, print_quit};

use lapin::{Connection, ConnectionProperties};
use risk_rust::gamelogic::gamestate::GameState;
use risk_rust::gamelogic::pause::handle_pause;
use risk_rust::pubsub::declare_and_bind;
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
    let sub_channel = match conn.create_channel().await {
        Ok(channel) => channel,
        Err(err) => {
            eprintln!("Error creating RabbitMQ channel: {}", err);
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
    let q = declare_and_bind(
        &sub_channel,
        routing::Exchange::PerilTopic.as_str(),
        routing::RoutingKey::Pause(String::from(&username)),
        pubsub::SimpleQueueType::Durable,
    )
    .await
    .expect("Error binding the queue");

    let mut game_state = GameState::new(&*username);
    pubsub::subscribe::subscribe_json(&sub_channel, &q, |ps| handle_pause(&mut game_state, ps))
        .await
        .expect("TODO: panic message");
    loop {
        let word = get_input();

        if word.len() == 0 {
            continue;
        }

        match word[0].as_str() {
            "spawn" => {
                println!("Spawning a new player...");
                match GameState::command_spawn(&mut game_state.clone(), word) {
                    Ok(_) => {}
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            }
            "move" => {
                println!("Moving a player...");
                match GameState::command_move(&mut game_state.clone(), word) {
                    Ok(_) => {}
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            }
            "status" => {
                println!("Checking the status of the game...");
                GameState::command_status(&game_state)
            }
            "spam" => println!("Spamming not allowed yet!"),
            "help" => print_client_help(),
            "quit" => {
                print_quit();
                break;
            }
            _ => println!("Invalid command. Please try again."),
        }
    }
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

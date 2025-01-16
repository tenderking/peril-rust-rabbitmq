mod handler;

use risk_rust::gamelogic::gamelogic::{client_welcome, get_input, print_client_help, print_quit};

use crate::handler::handler_pause;
use lapin::{Connection, ConnectionProperties};
use risk_rust::gamelogic::gamestate::GameState;
use risk_rust::pubsub::declare_and_bind;
use risk_rust::{pubsub, routing};
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::iterator::Signals;
use std::sync::Mutex;
use std::sync::{mpsc, Arc};
use std::thread;

#[tokio::main]
async fn main() {
    const ADDR: &str = "amqp://guest:guest@localhost:5672/my_vhost";

    let (done_sender, done_receiver) = mpsc::channel();

    let username = match client_welcome().await {
        Ok(username) => username,
        Err(_err) => {
            return;
        }
    };
    print!("Hello, {:?}", &username);

    let game_state = Arc::new(Mutex::new(GameState::new(&*username)));
    let game_state_clone_2 = Arc::clone(&game_state);
    let game_state_clone_1 = Arc::clone(&game_state);
    let game_state_clone = Arc::clone(&game_state);

    tokio::spawn(async move {
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

        let queue_name: String =
            routing::RoutingKey::Pause(username.clone().as_str().parse().unwrap()).as_str();

        let _q = declare_and_bind(
            &sub_channel,
            routing::Exchange::PerilTopic.as_str(),
            &queue_name,
            &routing::RoutingKey::Pause(String::from("*")).as_str(),
            &pubsub::SimpleQueueType::Durable,
        )
        .await
        .expect("Error binding the queue");

        pubsub::subscribe::subscribe_json(
            &sub_channel,
            &queue_name.as_str(),
            handler_pause(game_state_clone_1),
        )
        .await
        .expect("TODO: panic message");
    });
  tokio::spawn(async move {
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

        let queue_name = {
            let game_state_guard = game_state.lock().unwrap(); // Lock the mutex
            routing::RoutingKey::ArmyMoves(
                game_state_guard
                    .player
                    .username
                    .clone()
                    .as_str()
                    .to_string(),
            )
            .as_str()
        };

        let _q = declare_and_bind(
            &sub_channel,
            routing::Exchange::PerilTopic.as_str(),
            &queue_name,
            &routing::RoutingKey::ArmyMoves(String::from("*")).as_str(),
            &pubsub::SimpleQueueType::Transient,
        )
        .await
        .expect("Error binding the queue");
        let game_state_clone = Arc::clone(&game_state_clone_2);
        pubsub::subscribe::subscribe_json(
            &sub_channel,
            &queue_name.as_str(),
            handler_pause(game_state_clone),
        )
        .await
        .expect("TODO: panic message");
    });

    loop {
        let word = get_input();

        if word.len() == 0 {
            continue;
        }

        match word[0].as_str() {
            "spawn" => {
                println!("Spawning a new player...");
                match game_state_clone.lock() {
                    Ok(mut game_state) => match game_state.command_spawn(word) {
                        Ok(_) => {}
                        Err(err) => {
                            println!("{}", err);
                        }
                    },
                    Err(err) => {
                        eprintln!("Failed to acquire lock: {}", err);
                    }
                }
            }
            "move" => {
                println!("Moving a player...");
                match game_state_clone.lock() {
                    Ok(mut game_state) => match game_state.command_move(word) {
                        Ok(_) => {}
                        Err(err) => {
                            println!("{}", err);
                        }
                    },
                    Err(err) => {
                        eprintln!("Failed to acquire lock: {}", err);
                    }
                }
            }
            "status" => {
                println!("Checking the status of the game...");
                match game_state_clone.lock() {
                    Ok(game_state) => {
                        game_state.command_status();
                    }
                    Err(err) => {
                        eprintln!("Failed to acquire lock: {}", err);
                    }
                }
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

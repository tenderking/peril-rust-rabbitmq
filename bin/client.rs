mod client_handler;

use risk_rust::gamelogic::gamelogic::{
    client_welcome, get_input, get_malicious_log, print_client_help, print_quit,
};

use crate::client_handler::{handler_moves, handler_pause, handler_war};
use chrono::Utc;
use lapin::{Connection, ConnectionProperties};
use risk_rust::gamelogic::gamedata::{ArmyMove, Unit};
use risk_rust::gamelogic::gamestate::GameState;
use risk_rust::gamelogic::logs::init_logger;
use risk_rust::pubsub::declare_and_bind;
use risk_rust::pubsub::publish::publish_json;
use risk_rust::routing::GameLog;
use risk_rust::{pubsub, routing};
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::iterator::Signals;
use std::sync::Mutex;
use std::sync::{mpsc, Arc};
use std::thread;

#[tokio::main]
async fn main() {
    init_logger();
    const ADDR: &str = "amqp://guest:guest@localhost:5672/my_vhost";

    let (done_sender, done_receiver) = mpsc::channel();

    let username = match client_welcome().await {
        Ok(username) => username,
        Err(_err) => {
            return;
        }
    };
    print!("Hello, {:?}\n", &username);

    let game_state = Arc::new(Mutex::new(GameState::new(&*username)));

    let game_state_clone_2 = Arc::clone(&game_state);
    let game_state_clone_1 = Arc::clone(&game_state);
    let game_state_clone = Arc::clone(&game_state);
    let game_state_clone_3 = Arc::clone(&game_state);

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
            handler_moves(game_state_clone),
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

        let _q = declare_and_bind(
            &sub_channel,
            routing::Exchange::PerilTopic.as_str(),
            &routing::RoutingKey::WarRecognition(String::new()).as_str(),
            &routing::RoutingKey::WarRecognition(String::from("*")).as_str(),
            &pubsub::SimpleQueueType::Transient,
        )
        .await
        .expect("Error binding the queue");
        let game_state_clone = Arc::clone(&game_state_clone_3);
        pubsub::subscribe::subscribe_json(
            &sub_channel,
            &routing::RoutingKey::WarRecognition(String::new()).as_str(),
            handler_war(game_state_clone),
        )
        .await
        .expect("TODO: panic message");
    });
    let conn = match Connection::connect(&ADDR, ConnectionProperties::default()).await {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Error connecting to RabbitMQ: {}", err);
            return;
        }
    };

    let publish_move_channel = match conn.create_channel().await {
        Ok(channel) => channel,
        Err(err) => {
            eprintln!("Error creating RabbitMQ channel: {}", err);
            return;
        }
    };
    loop {
        let word = get_input();

        if word.len() == 0 {
            continue;
        }

        match word[0].as_str() {
            "spawn" => {
                println!("Spawning a new player...");
                match game_state_clone.clone().lock() {
                    Ok(mut game_state) => match game_state.command_spawn(word.clone()) {
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
            "move" => match game_state_clone.clone().lock() {
                Ok(mut game_state) => {
                    println!("Moving a player...");
                    match game_state.command_move(word.clone()) {
                        Ok(..) => {
                            let mut units: Vec<Unit> = Vec::new();
                            for unit in game_state.get_player_snap().units {
                                units.push(unit.1.clone());
                            }
                            publish_json(
                                publish_move_channel.clone(),
                                routing::Exchange::PerilTopic,
                                &routing::RoutingKey::ArmyMoves(String::from(
                                    game_state.get_player_snap().username,
                                ))
                                .clone()
                                .as_str(),
                                ArmyMove {
                                    player: game_state.get_player_snap().clone(),
                                    units,
                                    to_location: game_state
                                        .get_player_snap()
                                        .units
                                        .get(&0)
                                        .unwrap()
                                        .location
                                        .clone(),
                                },
                            )
                            .await
                            .expect("TODO: panic message");
                        }
                        Err(err) => {
                            println!("{}", err);
                        }
                    }
                }
                Err(_) => {
                    eprintln!("Failed to acquire lock");
                }
            },
            "status" => {
                println!("Checking the status of the game...");
                match game_state_clone.clone().lock() {
                    Ok(game_state) => {
                        game_state.command_status();
                    }
                    Err(err) => {
                        eprintln!("Failed to acquire lock: {}", err);
                    }
                }
            }
            "spam" => {
                if word.len() >= 1 {
                    if let Ok(_num) = word[1].parse::<i32>() {
                        match game_state_clone.clone().lock() {
                            Ok(game_state) => {
                                for _i in 0..word[1].parse::<i32>().unwrap() {
                                    let message = get_malicious_log();
                                    publish_json(
                                        publish_move_channel.clone(),
                                        routing::Exchange::PerilTopic,
                                        &routing::RoutingKey::GameLog(String::from(
                                            game_state.get_player_snap().username,
                                        ))
                                        .clone()
                                        .as_str(),
                                        GameLog {
                                            current_time: Utc::now(),
                                            message,
                                            username: game_state.get_player_snap().username,
                                        },
                                    )
                                    .await
                                    .expect("TODO: panic message");
                                }
                            }
                            Err(_gerr) => {}
                        }
                    }
                }
            }
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

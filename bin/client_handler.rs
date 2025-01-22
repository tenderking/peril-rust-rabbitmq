use lapin::{Connection, ConnectionProperties};
use risk_rust::gamelogic::gamedata::{ArmyMove, RecognitionOfWar};
use risk_rust::gamelogic::gamemove::MoveOutCome;
use risk_rust::gamelogic::gamestate::GameState;
use risk_rust::gamelogic::war::WarOutCome;
use risk_rust::pubsub::publish::publish_json;
use risk_rust::pubsub::subscribe::AckType;
use risk_rust::routing;
use risk_rust::routing::PlayingState;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::runtime::Handle;

pub fn handler_pause(
    game_state: Arc<Mutex<GameState>>,
) -> impl FnMut(PlayingState) -> AckType + 'static {
    move |ps: PlayingState| {
        println!("> Pausing game");

        // Lock the mutex before modifying the game state
        if let Ok(mut game) = game_state.lock() {
            game.handle_pause(ps);
            AckType::Ack
        } else {
            eprintln!("Failed to acquire lock on game_state");
            AckType::NackRequeue
        }
    }
}

pub fn handler_moves(
    game_state: Arc<Mutex<GameState>>, // Using std::sync::Mutex
) -> impl FnMut(ArmyMove) -> AckType + Send + 'static {
    const ADDR: &str = "amqp://guest:guest@localhost:5672/my_vhost";

    let conn_future = async {
        Connection::connect(&ADDR, ConnectionProperties::default())
            .await
            .expect("Error connecting to RabbitMQ")
    };
    let ch_future = async {
        let conn = conn_future.await;
        conn.create_channel()
            .await
            .expect("Error creating RabbitMQ channel")
    };

    let ch = tokio::task::block_in_place(|| Handle::current().block_on(ch_future));
    move |army_move: ArmyMove| {
        println!(">");

        if let Ok(game) = game_state.lock() {
            // Lock using std::sync::Mutex
            match game.handle_move(&army_move) {
                MoveOutCome::Safe => AckType::NackDiscard,
                MoveOutCome::MakeWar => {
                    let ch_clone = ch.clone();
                    let attacker = army_move.player.clone();
                    let defender = game.player.clone();

                    // Capture the Tokio runtime handle before moving into the thread
                    let handle = Handle::current();

                    // Spawn a standard blocking thread
                    thread::spawn(move || {
                        // Use the Tokio runtime to run async code inside a blocking thread
                        handle.block_on(async {
                            if let Err(e) = publish_json(
                                ch_clone,
                                routing::Exchange::PerilTopic,
                                &routing::RoutingKey::WarRecognition(attacker.username.clone())
                                    .as_str(),
                                RecognitionOfWar { attacker, defender },
                            )
                            .await
                            {
                                eprintln!("publish_json failed: {:?}", e);
                            }
                        });
                    });

                    AckType::Ack
                }
                MoveOutCome::SamePlayer => AckType::NackDiscard,
            }
        } else {
            eprintln!("Failed to acquire lock on game_state");
            AckType::NackRequeue
        }
    }
}

pub fn handler_war(
    game_state: Arc<Mutex<GameState>>,
) -> impl FnMut(RecognitionOfWar) -> AckType + 'static {
    move |wr: RecognitionOfWar| {
        println!(">");
        let mut game = game_state.lock().unwrap();
        match game.handle_war(&wr).war_out_come {
            WarOutCome::NotInvolved => AckType::NackRequeue,
            WarOutCome::NoUnits => AckType::NackDiscard,
            WarOutCome::OpponentWon => AckType::Ack,
            WarOutCome::YouWon => AckType::Ack,
            WarOutCome::Draw => AckType::Ack,
        }
    }
}

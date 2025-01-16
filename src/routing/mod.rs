use lapin::ExchangeKind;
use lapin::ExchangeKind::{Direct, Topic};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlayingState {
    pub is_paused: bool,
}
#[derive(Serialize, Deserialize)]
pub struct GameLog {
    pub current_time: String,
    pub message: String,
    pub username: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RoutingKey {
    ArmyMoves(String),      // "army_moves.{game_id}"
    WarRecognition(String), // "war.{game_id}"
    Pause(String),          // "pause.{game_id}"
    GameLog(String),        // "game_logs.{game_id}"
}

impl RoutingKey {
    pub fn as_str(& self) -> String {
        match self {
            RoutingKey::ArmyMoves(game_id) => format!("army_moves.{}", game_id),
            RoutingKey::WarRecognition(game_id) => format!("war.{}", game_id),
            RoutingKey::Pause(game_id) => format!("pause.{}", game_id),
            RoutingKey::GameLog(game_id) => format!("game_logs.{}", game_id),
        }
    }
}

pub enum Exchange {
    PerilDirect,
    PerilTopic,
}

impl Exchange {
    pub fn as_str(&self) -> &'static str {
        match self {
            Exchange::PerilDirect => "peril_direct",
            Exchange::PerilTopic => "peril_topic",
        }
    }
    pub fn exchange_type(&self) -> ExchangeKind {
        match self {
            Exchange::PerilDirect => Direct,
            Exchange::PerilTopic => Topic,
        }
    }
}

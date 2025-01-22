use chrono::{DateTime, Utc};
use lapin::ExchangeKind;
use lapin::ExchangeKind::{Direct, Topic};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlayingState {
    pub is_paused: bool,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct GameLog {
    pub current_time: DateTime<Utc>,
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
    // Constants for base routing key values (without "DEFAULT")
    pub const BASE_ARMY_MOVES: &'static str = "army_moves";
    pub const BASE_WAR_RECOGNITION: &'static str = "war";
    pub const BASE_PAUSE: &'static str = "pause";
    pub const BASE_GAME_LOG: &'static str = "game_logs";

    pub fn as_str(&self) -> String {
        match self {
            RoutingKey::ArmyMoves(game_id) => format!("{}.{}", Self::BASE_ARMY_MOVES, game_id),
            RoutingKey::WarRecognition(game_id) => {
                format!("{}.{}", Self::BASE_WAR_RECOGNITION, game_id)
            }
            RoutingKey::Pause(game_id) => format!("{}.{}", Self::BASE_PAUSE, game_id),
            RoutingKey::GameLog(game_id) => format!("{}.{}", Self::BASE_GAME_LOG, game_id),
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

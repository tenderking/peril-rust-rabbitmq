
pub struct PlayingState {
    is_paused: bool,
}

pub struct GameLog{
    current_time: String,
    message: String,
    username: String,
}

pub const ARMY_MOVES_PREFIX: &str = "army_moves";
pub const WAR_RECOGNITIONS_PREFIX: &str = "war";
pub const PAUSE_KEY: &str = "pause";
pub const GAME_LOG_SLUG: &str = "game_logs";

pub const EXCHANGE_PERIL_DIRECT: &str = "peril_direct";
pub const EXCHANGE_PERIL_TOPIC: &str = "peril_topic";
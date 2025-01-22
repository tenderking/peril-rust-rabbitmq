use risk_rust::gamelogic::logs::write_log;
use risk_rust::pubsub::subscribe::AckType;
use risk_rust::routing::GameLog;

pub fn handler_game_logs() -> impl Fn(GameLog) -> AckType {
    |game_log: GameLog| {
        println!(">");
        match write_log(game_log) {
            Ok(_) => AckType::Ack,
            Err(_) => AckType::NackRequeue,
        }
    }
}

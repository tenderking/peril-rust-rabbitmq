use std::sync::{Arc, Mutex};
use risk_rust::gamelogic::gamedata::ArmyMove;
use risk_rust::gamelogic::gamestate::GameState;

use risk_rust::routing::PlayingState;


pub fn handler_pause(game_state: Arc<Mutex<GameState>>) -> impl FnMut(PlayingState) + Send + 'static {
    move |ps: PlayingState| {
        println!("> Pausing game");

        // Lock the mutex before modifying the game state
        if let Ok(mut game) = game_state.lock() {
            game.handle_pause(ps);
        } else {
            eprintln!("Failed to acquire lock on game_state");
        }
    }
}

pub fn handler_moves<'a>(game_state: &'a GameState) -> impl FnMut(ArmyMove) + 'a {
    move |army_move: ArmyMove| {
        println!(">");
        game_state.handle_move( &army_move);
    }
}

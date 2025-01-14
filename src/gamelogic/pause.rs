use crate::gamelogic::gamestate::GameState;
use crate::routing::PlayingState;

pub fn handle_pause( game_state: &mut GameState, playing_state: PlayingState) {
    println!("-------------------------");
    if playing_state.is_paused{
        println!("==== Pause Detected ====");
        game_state.pause_game()
    }else {
        println!("==== Resume Detected ====");
        game_state.resume_game()
    }
}
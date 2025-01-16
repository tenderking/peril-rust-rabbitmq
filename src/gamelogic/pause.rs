use crate::gamelogic::gamestate::GameState;
use crate::routing::PlayingState;

impl GameState {
    pub fn handle_pause( &mut self, playing_state: PlayingState) {
        println!("-------------------------");
        if playing_state.is_paused {
            println!("==== Pause Detected ====");
            self.pause_game()
        } else {
            println!("==== Resume Detected ====");
            self.resume_game()
        }
    }
}
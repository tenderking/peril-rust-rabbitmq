use risk_rust::gamelogic::gamestate::GameState;
use risk_rust::gamelogic::pause::handle_pause;
use risk_rust::routing::PlayingState;

pub fn handler_pause<'a>(
    game_state: &'a mut GameState,
) -> impl FnMut(PlayingState) + 'a {
    move |ps: PlayingState| {
        handle_pause(game_state, ps);
    }
}
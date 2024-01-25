use crate::game_state::GameState;

#[derive(Debug)]
pub struct MarkovState {
	probability: f64,
	state: GameState,
}

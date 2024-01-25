use crate::{action::Action, game_state::{GameState, Turn}};

#[derive(Debug)]
pub struct MarkovState {
	probability: f64,
	state: GameState,
}

impl MarkovState {
	/// Finds the immediate next states. Their probabilities will assume we're
	/// at the current state now.
	pub fn next_states(&self, action: Action) -> Vec<MarkovState> {
		// Check if the state is a win or loss. If so, the state cannot change.
		if self.state.win || self.state.lose {
			return vec![MarkovState { probability: 1.0, state: self.state.clone() }]
		}

		// Take the action
		match action {
			Action::ShootDealer => {
				let live_probability = self.state.live_probability();
				vec![
					// It was live
					MarkovState { probability: live_probability, state: GameState {
						dealer_lives: self.state.dealer_lives - 1,
						lose: self.state.lose,
						num_blank: self.state.num_blank,
						num_live: self.state.num_live - 1,
						player_lives: self.state.player_lives,
						turn: Turn::Dealer,
						win: self.state.dealer_lives < 2
					}},
					// It was a blank
					MarkovState { probability: 1.0 - live_probability, state: GameState {
						dealer_lives: self.state.dealer_lives,
						lose: self.state.lose,
						num_blank: self.state.num_blank - 1,
						num_live: self.state.num_live,
						player_lives: self.state.player_lives,
						turn: Turn::Player,
						win: self.state.win
					}},
				]
			},
			Action::ShootPlayer => {
				let live_probability = self.state.live_probability();
				vec![
					// It was live
					MarkovState { probability: live_probability, state: GameState {
						dealer_lives: self.state.dealer_lives,
						lose: self.state.player_lives < 2,
						num_blank: self.state.num_blank,
						num_live: self.state.num_live - 1,
						player_lives: self.state.player_lives - 1,
						turn: Turn::Dealer,
						win: self.state.win
					}},
					// It was a blank
					MarkovState { probability: 1.0 - live_probability, state: GameState {
						dealer_lives: self.state.dealer_lives,
						lose: self.state.lose,
						num_blank: self.state.num_blank - 1,
						num_live: self.state.num_live,
						player_lives: self.state.player_lives,
						turn: Turn::Player,
						win: self.state.win
					}},
				]
			},
		}

	}
}

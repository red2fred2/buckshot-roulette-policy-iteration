pub static MAX_LIVES: u8 = 4;
pub static MAX_SHELLS: u8 = 8;

/// Relevant game state information
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct GameState {
	dealer_lives: u8,
	num_blank: u8,
	num_live: u8,
	player_lives: u8,
}

impl GameState {
	/// Find the probability that a live shell will be fired next
	pub fn live_probability(&self) -> f64 {
		self.num_live as f64 / (self.num_live as f64 + self.num_blank as f64)
	}
}

pub static MAX_LIVES: u8 = 4;
pub static MAX_SHELLS: u8 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Turn {
	Dealer,
	Player,
}

/// Relevant game state information
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameState {
	pub dealer_lives: u8,
	pub lose: bool,
	pub num_blank: u8,
	pub num_live: u8,
	pub player_lives: u8,
	pub turn: Turn,
	pub win: bool,
}

impl GameState {
	/// Find the probability that a live shell will be fired next
	pub fn live_probability(&self) -> f64 {
		self.num_live as f64 / (self.num_live as f64 + self.num_blank as f64)
	}
}

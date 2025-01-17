mod rank;
mod suit;

pub use rank::Rank;
pub use suit::Suit;

struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new() -> Self {
        Self {
            suit: Suit::Heart,
            rank: Rank::Six,
        }
    }
}

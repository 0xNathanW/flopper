pub mod card;
pub mod deck;
pub mod hand;
pub mod board;
pub mod evaluate;
pub mod equity;
pub mod range;
pub mod tables;
pub mod error;
pub mod isomorphism;

// Primitives.
pub use {
    hand::Hand,
    deck::Deck,
    card::Card,
    card::Suit,
    range::Range,
    board::Board,
};

// Remove combos conflicting with board and dead cards.
pub fn remove_dead(ranges: Vec<Range>, board: &[Card]) -> error::Result<(Vec<Vec<(Hand, f32)>>, Deck)> {
    
    let mut deck = Deck::new();
    let mut removed = 0_u64;

    // Build the removed cards bitmask
    for card in board {
        removed |= 1 << card.0;
    }
    
    // Remove all cards from the deck in a single pass
    deck.remove_dead(removed);
        
    let hands = ranges
        .iter()
        .map(|range| {
            range.hand_combos(removed)
        }).collect();

    Ok((hands, deck))
}

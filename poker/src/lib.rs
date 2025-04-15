
pub mod card;
pub mod deck;
pub mod hand;
pub mod board;
pub mod evaluate;
pub mod equity;
pub mod range;
pub mod tables;
pub mod error;

// Primitives.
pub use {
    hand::Hand,
    deck::Deck,
    card::Card,
    range::Range,
    board::Board,
};

// Remove combos conflicting with board and dead cards.
pub fn remove_dead(ranges: Vec<Range>, board: &[Card]) -> error::Result<(Vec<Vec<(Hand, f32)>>, Deck)> {
    
    let mut deck = Deck::new();
    let mut removed = 0_u64;

    board.iter().for_each(|card| {
        deck.remove(card);
        removed |= 1 << card.0;
    });
        
    let hands = ranges
        .iter()
        .map(|range| {
            let hands = range.hand_combos_dead(removed);
            hands
        }).collect();

    Ok((hands, deck))
}
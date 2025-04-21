use std::ops::Index;
use rand::prelude::*;
use crate::card::Card;

#[derive(Debug, Clone)]
pub struct Deck(Vec<Card>);

impl Default for Deck {
    fn default() -> Deck {
        Deck::new()
    }
}

impl Deck {

    pub fn new() -> Deck {
        Deck((0..52_u8).map(|i| i.into()).collect())
    }

    pub fn shuffle<R: Rng>(&mut self, rng: &mut R) {
        self.0.shuffle(rng);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.0.pop()
    }

    pub fn push(&mut self, card: Card) {
        self.0.push(card);
    }

    pub fn remove(&mut self, card: &Card) {
        self.0.retain(|c| c != card);
    }
}

impl Index<usize> for Deck {
    type Output = Card;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

// Implement the IntoIterator trait for a reference to Deck
impl<'a> IntoIterator for &'a Deck {
    type Item = &'a Card;
    type IntoIter = std::slice::Iter<'a, Card>;

    fn into_iter(self) -> Self::IntoIter {
        (self.0).iter()
    }
}

impl AsRef<[Card]> for Deck {
    fn as_ref(&self) -> &[Card] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Rank;

    #[test]
    fn test_new_deck() {
        let mut deck = Deck::new();
        assert_eq!(deck.len(), 52);
        deck.0.sort_by(|a, b| a.rank().cmp(&b.rank()));
        assert!(deck[0].rank() == Rank::Two);
        assert!(deck[51].rank() == Rank::Ace);
    }
}
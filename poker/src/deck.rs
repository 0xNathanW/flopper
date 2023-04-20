use std::ops::Index;
use rand::prelude::*;
use crate::card::{Card,RANKS, SUITS};

#[derive(Debug, Clone)]
pub struct Deck<T>(Vec<T>);

impl Deck<Card> {
    pub fn new() -> Deck<Card> {
        let mut deck = Vec::new();
        for suit in SUITS.iter() {
            for rank in RANKS.iter() {
                deck.push(Card::new(*rank, *suit));
            }
        }
        Deck(deck)
    }

    pub fn new_shuffled() -> Deck<Card> {
        let mut deck = Deck::<Card>::new();
        deck.shuffle();
        deck
    }
}

impl Deck<usize> {
    // For use in two-plus-two hand evaluator.
    pub fn new() -> Deck<usize> {
        let mut deck = Vec::new();
        for suit in SUITS.iter() {
            for rank in RANKS.iter() {
                deck.push(Card::new(*rank, *suit).idx());
            }
        }
        Deck(deck)
    }

    pub fn new_shuffled() -> Deck<usize> {
        let mut deck = Deck::<usize>::new();
        deck.shuffle();
        deck
    }
}

impl Deck<u32> {
    // For use in two-plus-two hand evaluator.
    pub fn new() -> Deck<u32> {
        let mut deck = Vec::new();
        for suit in SUITS.iter() {
            for rank in RANKS.iter() {
                deck.push(Card::new(*rank, *suit).bit_mask());
            }
        }
        Deck(deck)
    }

    pub fn new_shuffled() -> Deck<u32> {
        let mut deck = Deck::<u32>::new();
        deck.shuffle();
        deck
    }
}

impl<T: PartialEq> Deck<T> {

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.0.shuffle(&mut rng);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn draw(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn draw_n(&mut self, n: usize) -> Option<Vec<T>> {
        if n > self.len() {
            return None;
        }
        let mut cards = Vec::new();
        for _ in 0..n {
            cards.push(self.draw().unwrap());
        }
        Some(cards)
    }

    pub fn remove_dead(&mut self, dead: T) {
        self.0.retain(|c| *c != dead);
    }
}

impl<T> Index<usize> for Deck<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

// Implement the IntoIterator trait for a reference to Deck
impl<'a, T> IntoIterator for &'a Deck<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).iter()
    }
}

impl<T> AsRef<[T]> for Deck<T> {
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Rank;

    #[test]
    fn test_new_deck() {
        let mut deck = Deck::<Card>::new();
        assert_eq!(deck.len(), 52);
        deck.0.sort_by(|a, b| a.rank().cmp(&b.rank()));
        assert!(deck[0].rank() == Rank::Two);
        assert!(deck[51].rank() == Rank::Ace);
    }
}
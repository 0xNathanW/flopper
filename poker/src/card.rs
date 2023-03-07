use std::{fmt::{Display, Debug}, ops::Index};
use rand::prelude::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Suit { 
    Hearts, 
    Diamonds,
    Clubs, 
    Spades,
}

const SUITS: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

impl From<u8> for Suit {
    fn from(value: u8) -> Self {
        match value {
            0 => Suit::Hearts,
            1 => Suit::Diamonds,
            2 => Suit::Clubs,
            3 => Suit::Spades,
            _ => panic!("Invalid suit value: {}", value),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Hearts   => write!(f, "♥"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Clubs    => write!(f, "♣"),
            Suit::Spades   => write!(f, "♠"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

const RANKS: [Rank; 13] = [
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
    Rank::Ace,
];

impl From<u8> for Rank {
    fn from(value: u8) -> Self {
        match value {
            0 => Rank::Two,
            1 => Rank::Three,
            2 => Rank::Four,
            3 => Rank::Five,
            4 => Rank::Six,
            5 => Rank::Seven,
            6 => Rank::Eight,
            7 => Rank::Nine,
            8 => Rank::Ten,
            9 => Rank::Jack,
            10 => Rank::Queen,
            11 => Rank::King,
            12 => Rank::Ace,
            _ => panic!("Invalid rank value: {}", value),
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rank::Two => write!(f, "2"),
            Rank::Three => write!(f, "3"),
            Rank::Four => write!(f, "4"),
            Rank::Five => write!(f, "5"),
            Rank::Six => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
            Rank::Nine => write!(f, "9"),
            Rank::Ten => write!(f, "10"),
            Rank::Jack => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King => write!(f, "K"),
            Rank::Ace => write!(f, "A"),
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Card(u8);

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card((rank as u8) << 4 | (suit as u8))
    }

    pub fn suit(&self) -> Suit {
        (self.0 & 0b0000_0011).into()
    }

    pub fn rank(&self) -> Rank {
        (self.0 >> 4).into()
    }

    pub fn chen_score(&self) -> u8 {
        match self.rank() {
            Rank::Ace => 10,
            Rank::King => 8,
            Rank::Queen => 7,
            Rank::Jack => 6,
            _ => self.rank() as u8 / 2,
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.rank(), self.suit())
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.rank(), self.suit())
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank() == other.rank()
    }
}

impl Eq for Card {}

// Ordering of cards is based on rank only.
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.rank().cmp(&other.rank()))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank().cmp(&other.rank())
    }
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        
        assert!(value.len() == 2, "Invalid card string: {}", value);
        let mut chars = value.chars();

        let rank = match chars.next().unwrap() {
            '2' => Rank::Two,
            '3' => Rank::Three,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            '9' => Rank::Nine,
            'T' | 't'  => Rank::Ten,
            'J' | 'j'  => Rank::Jack,
            'Q' | 'q'  => Rank::Queen,
            'K' | 'k'  => Rank::King,
            'A' | 'a'  => Rank::Ace,
            // Regex should prevent this from happening.
            _ => unreachable!("Invalid rank character: {}", value),
        };

        let suit = match chars.next().unwrap() {
            'd' | 'D'  => Suit::Diamonds,
            'h' | 'H'  => Suit::Hearts,
            'c' | 'C'  => Suit::Clubs,
            's' | 'S'  => Suit::Spades,
            // Regex should prevent this from happening.
            _ => unreachable!("Invalid suit character: {}", value),
        };

        Card::new(rank, suit)
    }
}

pub struct Deck(Vec<Card>);

impl Deck {
    pub fn new() -> Deck {
        let mut deck = Vec::new();
        for suit in SUITS.iter() {
            for rank in RANKS.iter() {
                deck.push(Card::new(*rank, *suit));
            }
        }
        Deck(deck)
    }

    pub fn new_shuffled() -> Deck {
        let mut deck = Deck::new();
        deck.shuffle();
        deck
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.0.shuffle(&mut rng);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.0.pop()
    }
}

impl Index<usize> for Deck {
    type Output = Card;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_deck() {
        let mut deck = Deck::new();
        assert_eq!(deck.len(), 52);
        deck.0.sort_by(|a, b| a.rank().cmp(&b.rank()));
        assert!(deck[0].rank() == Rank::Two);
        assert!(deck[51].rank() == Rank::Ace);
    }

    #[test]
    fn test_ordering() {
        let card_a = Card::new(Rank::Ace, Suit::Clubs);
        let card_b = Card::new(Rank::King, Suit::Clubs);
        assert!(card_a > card_b);
        assert!(card_a >= card_b);
        assert!(card_b < card_a);
        assert!(card_b <= card_a);
        assert!(card_a != card_b);

        let card_a = Card::new(Rank::Nine, Suit::Clubs);
        let card_b = Card::new(Rank::Nine, Suit::Diamonds);
        assert!(card_a == card_b);
        assert!(card_a <= card_b);
        assert!(card_a >= card_b);
        assert!(card_b <= card_a);
        assert!(card_b >= card_a);
    }

    #[test]
    fn test_size() {
        assert!(std::mem::size_of::<Card>() == 1);
    }

    #[test]
    fn test_from_str() {
        let card = Card::new(Rank::Ace, Suit::Hearts);
        assert!(card.rank() == Rank::Ace);
        assert!(card.suit() == Suit::Hearts);

        let card = Card::new(Rank::Two, Suit::Clubs);
        assert!(card.rank() == Rank::Two);
        assert!(card.suit() == Suit::Clubs);
    }

    #[test]
    fn test_suit_u8() {
        let card = Card::new(Rank::Ace, Suit::Hearts);
        assert!(card.suit() as u8 == 0);

        let card = Card::new(Rank::Ace, Suit::Clubs);
        assert!(card.suit() as u8 == 2);
    }
}

mod naive;
mod two_plus_two;
mod senzee;
mod tables;

pub use naive::{
    rank_hand_naive, 
    rank_cards_naive
};
pub use senzee::{
    rank_hand_senzee, 
    rank_cards_senzee, 
    rank_bit_mask_senzee
};
pub use two_plus_two::{
    rank_hand_two_plus_two,
    rank_hand_5,
    rank_hand_6,
    rank_hand_7,
    load_lookup_table,
    save_lookup_table,
    generate_lookup_table,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum HandRank {
    HighCard(u32),
    Pair(u32),
    TwoPair(u32),
    ThreeOfAKind(u32),
    Straight(u32),
    Flush(u32),
    FullHouse(u32),
    FourOfAKind(u32),
    StraightFlush(u32),
}

impl From<u16> for HandRank {
    fn from(value: u16) -> Self {
        match value {
            0..=1276 => HandRank::HighCard(value as u32),
            1277..=4136 => HandRank::Pair(value as u32 - 1277),
            4137..=4994 => HandRank::TwoPair(value as u32 - 4137),
            4995..=5852 => HandRank::ThreeOfAKind(value as u32 - 4995),
            5853..=5862 => HandRank::Straight(value as u32 - 5853),
            5863..=7139 => HandRank::Flush(value as u32 - 5863),
            7140..=7295 => HandRank::FullHouse(value as u32 - 7140),
            7296..=7451 => HandRank::FourOfAKind(value as u32 - 7296),
            7452..=7461 => HandRank::StraightFlush(value as u32 - 7452),
            _ => panic!("Unexpected hand rank value! '{}'", value)
        }
    }
}

impl HandRank {
    pub fn rank_variant(value: HandRank) -> HandRank {
        match value {
            HandRank::HighCard(_) => HandRank::HighCard(0),
            HandRank::Pair(_) => HandRank::Pair(0),
            HandRank::TwoPair(_) => HandRank::TwoPair(0),
            HandRank::ThreeOfAKind(_) => HandRank::ThreeOfAKind(0),
            HandRank::Straight(_) => HandRank::Straight(0),
            HandRank::Flush(_) => HandRank::Flush(0),
            HandRank::FullHouse(_) => HandRank::FullHouse(0),
            HandRank::FourOfAKind(_) => HandRank::FourOfAKind(0),
            HandRank::StraightFlush(_) => HandRank::StraightFlush(0),
        }
    }
}

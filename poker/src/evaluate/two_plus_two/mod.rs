use crate::card::Card;
use super::HandRank;

mod generate;
mod eval;

pub use generate::{
    generate_lookup_table,
    save_lookup_table,
    load_lookup_table,
};

pub fn rank_hand_two_plus_two(hand: &[Card], lookup_table: &[i32]) -> HandRank {
    assert!(hand.len() >= 5 && hand.len() <= 7);
    // Convert the cards to their index in the lookup table.
    let rank = match hand.len() {
        5 => rank_hand_5(hand, lookup_table),
        6 => rank_hand_6(hand, lookup_table),
        7 => rank_hand_7(hand, lookup_table),
        _ => unreachable!(),
    };

    HandRank::from(rank)
}

#[inline]
pub fn rank_hand_5(hand: &[Card], lookup_table: &[i32]) -> u16 {
    let mut r = lookup_table[53 + hand[0].0 as usize + 1] as usize;
    r = lookup_table[r + hand[1].0 as usize + 1] as usize;
    r = lookup_table[r + hand[2].0 as usize + 1] as usize;
    r = lookup_table[r + hand[3].0 as usize + 1] as usize;
    r = lookup_table[r + hand[4].0 as usize + 1] as usize;
    r = lookup_table[r] as usize;
    r as u16
}

#[inline]
pub fn rank_hand_6(hand: &[Card], lookup_table: &[i32]) -> u16 {
    let mut r = lookup_table[53 + hand[0].0 as usize + 1] as usize;
    r = lookup_table[r + hand[1].0 as usize + 1] as usize;
    r = lookup_table[r + hand[2].0 as usize + 1] as usize;
    r = lookup_table[r + hand[3].0 as usize + 1] as usize;
    r = lookup_table[r + hand[4].0 as usize + 1] as usize;
    r = lookup_table[r + hand[5].0 as usize + 1] as usize;
    r = lookup_table[r] as usize;
    r as u16
}

#[inline]
pub fn rank_hand_7(hand: &[Card], lookup_table: &[i32]) -> u16 {
    let mut r = lookup_table[53 + hand[0].0 as usize + 1] as usize;
    r = lookup_table[r + hand[1].0 as usize + 1] as usize;
    r = lookup_table[r + hand[2].0 as usize + 1] as usize;
    r = lookup_table[r + hand[3].0 as usize + 1] as usize;
    r = lookup_table[r + hand[4].0 as usize + 1] as usize;
    r = lookup_table[r + hand[5].0 as usize + 1] as usize;
    r = lookup_table[r + hand[6].0 as usize + 1] as usize;
    r as u16
}
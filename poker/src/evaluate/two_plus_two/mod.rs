use crate::{card::Card, hand::HandRank};

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
    let hand_idxs = hand.iter().map(|&c| c.idx()).collect::<Vec<usize>>();
    rank_idx_two_plus_two(&hand_idxs, lookup_table)
}

#[inline]
pub fn rank_cards_two_plus_two(hand: &[Card], lookup_table: &[i32]) -> HandRank {
    assert!(hand.len() >= 5 && hand.len() <= 7);
    // Convert the cards to their index in the lookup table.
    let hand_idxs = hand.iter().map(|&c| c.idx()).collect::<Vec<usize>>();
    rank_idx_two_plus_two(&hand_idxs, lookup_table)
}

#[inline]
pub fn rank_idx_two_plus_two(hand: &[usize], lookup_table: &[i32]) -> HandRank {
    assert!(hand.len() >= 5 && hand.len() <= 7);
    
    let rank = match hand.len() {
        5 => rank_hand_5(hand, lookup_table),
        6 => rank_hand_6(hand, lookup_table),
        7 => rank_hand_7(hand, lookup_table),
        _ => unreachable!(),
    };

    HandRank::from(rank)
}

#[inline]
fn rank_hand_5(hand: &[usize], lookup_table: &[i32]) -> u16 {
    let mut r = lookup_table[53 + hand[0]] as usize;
    r = lookup_table[r + hand[1]] as usize;
    r = lookup_table[r + hand[2]] as usize;
    r = lookup_table[r + hand[3]] as usize;
    r = lookup_table[r + hand[4]] as usize;
    r = lookup_table[r] as usize;
    r as u16
}

#[inline]
fn rank_hand_6(hand: &[usize], lookup_table: &[i32]) -> u16 {
    let mut r = lookup_table[53 + hand[0]] as usize;
    r = lookup_table[r + hand[1]] as usize;
    r = lookup_table[r + hand[2]] as usize;
    r = lookup_table[r + hand[3]] as usize;
    r = lookup_table[r + hand[4]] as usize;
    r = lookup_table[r + hand[5]] as usize;
    r = lookup_table[r] as usize;
    r as u16
}

#[inline]
fn rank_hand_7(hand: &[usize], lookup_table: &[i32]) -> u16 {
    let mut r = lookup_table[53 + hand[0]] as usize;
    r = lookup_table[r + hand[1]] as usize;
    r = lookup_table[r + hand[2]] as usize;
    r = lookup_table[r + hand[3]] as usize;
    r = lookup_table[r + hand[4]] as usize;
    r = lookup_table[r + hand[5]] as usize;
    r = lookup_table[r + hand[6]] as usize;
    r as u16
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Deck;
    use crate::hand::HandRank;
    use std::collections::HashMap;

    fn setup_lookup_table() -> Vec<i32> {
        match load_lookup_table() {
            Ok(lookup_table) => lookup_table,
            Err(e) => {
                println!("Error loading lookup table: {}", e);
                let lookup_table = generate_lookup_table();
                lookup_table
            }
        }
    }

   #[test]
   fn test_combo_7_two_plus_two() {
    
    let lookup_table = setup_lookup_table();
    let cards = Deck::new().into_iter().map(|c| c.idx()).collect::<Vec<usize>>(); 
    let mut rank_count: HashMap<HandRank, usize> = HashMap::new();

    let mut hand = [0_usize; 7];
    for a in 0..52 {
        for b in (a + 1)..52 {
            for c in (b + 1)..52 {
                for d in (c + 1)..52 {
                    for e in (d + 1)..52 {
                        for f in (e + 1)..52 {
                            for g in (f + 1)..52 {
                                
                                hand[0] = cards[a];
                                hand[1] = cards[b];
                                hand[2] = cards[c];
                                hand[3] = cards[d];
                                hand[4] = cards[e];
                                hand[5] = cards[f];
                                hand[6] = cards[g];

                                let rank = rank_idx_two_plus_two(&hand, &lookup_table);
                                
                                rank_count
                                    .entry(HandRank::rank_variant(rank))
                                    .and_modify(|count| *count += 1)
                                    .or_insert(1);
                            }
                        }
                    }
                }
            }
        }
    }

    assert_eq!(*rank_count.get(&HandRank::HighCard(0)).unwrap(), 23294460);
    assert_eq!(*rank_count.get(&HandRank::Pair(0)).unwrap(), 58627800);
    assert_eq!(*rank_count.get(&HandRank::TwoPair(0)).unwrap(), 31433400);
    assert_eq!(*rank_count.get(&HandRank::ThreeOfAKind(0)).unwrap(), 6461620);
    assert_eq!(*rank_count.get(&HandRank::Straight(0)).unwrap(), 6180020);
    assert_eq!(*rank_count.get(&HandRank::Flush(0)).unwrap(), 4047644);
    assert_eq!(*rank_count.get(&HandRank::FullHouse(0)).unwrap(), 3473184);
    assert_eq!(*rank_count.get(&HandRank::FourOfAKind(0)).unwrap(), 224848);
    assert_eq!(*rank_count.get(&HandRank::StraightFlush(0)).unwrap(), 41584);
   }
}

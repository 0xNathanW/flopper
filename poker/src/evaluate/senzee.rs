use crate::card::Card;

use super::tables::*;

// fn find(u: u32) -> u32 {
//     let mut u = u;
//     u += 0xe91aaa35;
//     u ^= u >> 16;
//     u += u << 8;
//     u ^= u >> 4;
//     let b = (u >> 8) & 0x1ff;
//     let a = (u + (u << 2)) >> 19;
//     a ^ HASH_ADJUST[b as usize] as u32
// }

fn find(u: usize) -> usize {
    
    let mut low = 0;
    let mut high = 4887;
    let mut mid;

    while low <= high {
        mid = (low + high) >> 1;

        if u < PRODUCTS[mid] as usize {
            high = mid - 1;
        } else if u > PRODUCTS[mid] as usize {
            low = mid + 1;
        } else {
            return mid;
        }
    }

    unreachable!("find_alt failed to find a match for {}. Low {}, high {}", u, low, high);
}

pub fn eval_5(hand: &[i32]) -> u16 {
    assert!(hand.len() == 5);

    let q = (hand[0] | hand[1] | hand[2] | hand[3] | hand[4]) as usize >> 16;
    if (hand[0] & hand[1] & hand[2] & hand[3] & hand[4] & 0xF000) != 0 {
        return FLUSHES[q];
    }

    let s = UNIQUE_5[q];
    if s != 0 {
        return s;
    }

    let q = (hand[0] & 0xFF) * (hand[1] & 0xFF) * (hand[2] & 0xFF) * (hand[3] & 0xFF) * (hand[4] & 0xFF);
    let rank = VALUES[find(q as usize)];
    7461 - rank
}

pub fn eval_6(hand: &[i32]) -> u16 {
    assert!(hand.len() == 6);

    let mut tmp;
    let mut best = 0;
    let mut sub_hand = [0_i32; 5];

    for id in PERM_6.iter() {
        
        for i in 0..5 {
            sub_hand[i] = hand[id[i] as usize];
        }

        tmp = eval_5(&sub_hand);
        if tmp > best {
            best = tmp;
        }
    }

    best
}

pub fn eval_7(hand: &[i32]) -> u16 {
    assert!(hand.len() == 7);

    let mut tmp;
    let mut best = 0;
    let mut sub_hand = [0_i32; 5];

    for id in PERM_7.iter() {
        
        for i in 0..5 {
            sub_hand[i] = hand[id[i] as usize];
        }

        tmp = eval_5(&sub_hand);
        if tmp > best {
            best = tmp;
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    use crate::{card::Deck, hand::HandRank};

    #[test]
    fn test_5_card_combos_eval() {

        let deck = Deck::new();
        let cards = deck.into_iter().map(|card| card.bit_mask()).collect::<Vec<i32>>();

        let mut rank_count: HashMap<HandRank, usize> = HashMap::new();
        let mut rank_num_count: HashMap<u16, bool> = HashMap::new();
    
        let mut hand = [0_i32; 5];
        for a in 0..52 {
            for b in (a + 1)..52 {
                for c in (b + 1)..52 {
                    for d in (c + 1)..52 {
                        for e in (d + 1)..52 {

                            hand[0] = cards[a];
                            hand[1] = cards[b];
                            hand[2] = cards[c];
                            hand[3] = cards[d];
                            hand[4] = cards[e];
                            
                            for i in 0..5 {
                                print!("{:?} ", Card::from_bit_mask(hand[i]));
                            }

                            let rank = eval_5(&hand);
                            print!("-> {:?}\n", HandRank::from(rank));
                            rank_num_count.entry(rank).or_insert(true);
                        }
                    }
                }
            }
        }

        for key in rank_num_count.keys() {
            let rank_class: HandRank = (*key).into();
            let rank_class = match rank_class {
                HandRank::HighCard(_) => HandRank::HighCard(0),
                HandRank::Pair(_) => HandRank::Pair(0),
                HandRank::TwoPair(_) => HandRank::TwoPair(0),
                HandRank::ThreeOfAKind(_) => HandRank::ThreeOfAKind(0),
                HandRank::Straight(_) => HandRank::Straight(0),
                HandRank::Flush(_) => HandRank::Flush(0),
                HandRank::FullHouse(_) => HandRank::FullHouse(0),
                HandRank::FourOfAKind(_) => HandRank::FourOfAKind(0),
                HandRank::StraightFlush(_) => HandRank::StraightFlush(0),
            };
            let count = rank_count.entry(rank_class).or_insert(0);
            *count += 1;
        }

        assert_eq!(rank_num_count.len(), 7462);
        assert_eq!(*rank_count.get(&HandRank::HighCard(0)).unwrap(), 1277);
        assert_eq!(*rank_count.get(&HandRank::Pair(0)).unwrap(), 2860);
        assert_eq!(*rank_count.get(&HandRank::TwoPair(0)).unwrap(), 858);
        assert_eq!(*rank_count.get(&HandRank::ThreeOfAKind(0)).unwrap(), 858);
        assert_eq!(*rank_count.get(&HandRank::Straight(0)).unwrap(), 10);
        assert_eq!(*rank_count.get(&HandRank::Flush(0)).unwrap(), 1277);
        assert_eq!(*rank_count.get(&HandRank::FullHouse(0)).unwrap(), 156);
        assert_eq!(*rank_count.get(&HandRank::FourOfAKind(0)).unwrap(), 156);
        assert_eq!(*rank_count.get(&HandRank::StraightFlush(0)).unwrap(), 10);
    }
}
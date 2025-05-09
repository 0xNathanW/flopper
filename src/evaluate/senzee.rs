use crate::{card::Card, tables::*, error::{Error, Result}};
use super::HandRank;

pub fn rank_hand_senzee(hand: &[Card]) -> Result<HandRank> {
    let cards = hand.iter().map(|&c| c.bit_mask()).collect::<Vec<u32>>();
    rank_bit_mask_senzee(&cards)
}


pub fn rank_bit_mask_senzee(hand: &[u32]) -> Result<HandRank> {
    let rank_num = match hand.len() {
        5 => eval_5_senzee(hand),
        6 => eval_6_senzee(hand),
        7 => eval_7_senzee(hand),
        _ => return Err(Error::InvalidHandSize(hand.len())),
    };

    Ok(HandRank::from(rank_num))
}

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

// Raw senzee evaluation functions, used in lookup generation.

pub fn eval_5_senzee(hand: &[u32]) -> u16 {
    assert!(hand.len() == 5);

    let q = ((hand[0] | hand[1] | hand[2] | hand[3] | hand[4]) as usize) >> 16;
    if (hand[0] & hand[1] & hand[2] & hand[3] & hand[4] & 0xF000) != 0 {
        return 7461 - (FLUSHES[q] - 1);
    }

    let s = UNIQUE_5[q];
    if s != 0 {
        return 7461 - (s - 1);
    }

    let q = (hand[0] & 0xFF) * (hand[1] & 0xFF) * (hand[2] & 0xFF) * (hand[3] & 0xFF) * (hand[4] & 0xFF);
    let rank = VALUES[find(q as usize)];
    7461 - (rank - 1)
}

pub fn eval_6_senzee(hand: &[u32]) -> u16 {
    assert!(hand.len() == 6);

    let mut tmp;
    let mut best = 0;
    let mut sub_hand = [0_u32; 5];

    for id in PERM_6.iter() {
        
        for i in 0..5 {
            sub_hand[i] = hand[id[i] as usize];
        }

        tmp = eval_5_senzee(&sub_hand);
        if tmp > best {
            best = tmp;
        }
    }

    best
}

pub fn eval_7_senzee(hand: &[u32]) -> u16 {
    assert!(hand.len() == 7);

    let mut tmp;
    let mut best = 0;
    let mut sub_hand = [0_u32; 5];

    for id in PERM_7.iter() {
        
        for i in 0..5 {
            sub_hand[i] = hand[id[i] as usize];
        }

        tmp = eval_5_senzee(&sub_hand);
        if tmp > best {
            best = tmp;
        }
    }

    best
}

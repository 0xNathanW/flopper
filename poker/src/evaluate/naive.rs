use crate::card::Card;
use super::HandRank;

#[inline]
pub fn rank_cards_naive(hand: &[Card]) -> HandRank {
    assert!(hand.len() >= 5 && hand.len() <= 7);

    match hand.len() {
        5 => rank_hand_5(&hand),
        6..=7 => rank_hand_not_5(&hand),
        _ => unreachable!(),
    }
}

fn rank_hand_not_5(cards: &[Card]) -> HandRank {

    let mut rank_set = 0_u32;
    let mut suit_set = [0_u32; 4];

    let mut rank_to_count = [0_u8; 13];
    let mut count_to_rank = [0_u32; 7];

    // Add the cards on the board to the set of cards.
    for card in cards.iter() {
        rank_set |= 1 << (card.rank() as u8);
        rank_to_count[card.rank() as usize] += 1;
        suit_set[card.suit() as usize] |= 1 << (card.rank() as u8);
    }

    for (rank, &count) in rank_to_count.iter().enumerate() {
        count_to_rank[count as usize] |= 1 << rank;
    }

    let flush = suit_set.iter().position(|&suit| suit.count_ones() >= 5);

    // Flush/straight flush.
    if let Some(flush) = flush {
        // Check for straight flush.
        if let Some(straight_flush) = find_straight(suit_set[flush]) {
            HandRank::StraightFlush(straight_flush)
        } else {
            // Normal flush.
            HandRank::Flush(n_msb(suit_set[flush], 5))
        }
    }

    // Four of a kind.
    else if count_to_rank[4] != 0 {
        HandRank::FourOfAKind(count_to_rank[4] << 13 | msb(rank_set ^ count_to_rank[4]))
    }
    
    // Full house.
    else if count_to_rank[3] != 0 && count_to_rank[3].count_ones() == 2 {
        let set = msb(count_to_rank[3]);
        let pair = count_to_rank[3] ^ set;
        HandRank::FullHouse(set << 13 | pair)
    } 
    else if count_to_rank[3] != 0 && count_to_rank[2] != 0 {
        let set = count_to_rank[3];
        let pair = msb(count_to_rank[2]);
        HandRank::FullHouse(set << 13 | pair)

    // Straight.
    } else if let Some(straight) = find_straight(rank_set) {
        HandRank::Straight(straight)
    }

    // Three of a kind.
    else if count_to_rank[3] != 0 {
        HandRank::ThreeOfAKind(count_to_rank[3] << 13 | n_msb(rank_set ^ count_to_rank[3], 2))
    }

    // Two pair.
    else if count_to_rank[2].count_ones() >=2 {
        let pairs = n_msb(count_to_rank[2], 2);
        HandRank::TwoPair(pairs << 13 | msb(rank_set ^ pairs))
    }

    // High Card.
    else if count_to_rank[2] == 0 {
        HandRank::HighCard(n_msb(rank_set, 5))
    }

    // Pair.
    else {
        HandRank::Pair(count_to_rank[2] << 13 | n_msb(rank_set ^ count_to_rank[2], 3))
    }
}

// Faster version of bits_hand_rank that only works for 5 card hands.
fn rank_hand_5(cards: &[Card]) -> HandRank {
    assert!(cards.len() == 5);

    let mut rank_set = 0_u32;
    let mut suit_set = 0_u32;

    let mut rank_to_count = [0_u8; 13];
    let mut count_to_rank = [0_u32; 5];

    // Add the cards on the board to the set of cards.
    for card in cards.iter() {
        suit_set |= 1 << (card.suit() as u8);
        rank_set |= 1 << (card.rank() as u8);
        rank_to_count[card.rank() as usize] += 1;

    }

    for (rank, &count) in rank_to_count.iter().enumerate() {
        count_to_rank[count as usize] |= 1 << rank as u32;
    }

    let unique_ranks = rank_set.count_ones();

    match unique_ranks {
        // 5 unique ranks => straight, flush, or just high card.
        5 => {
            // If all same suit, then flush.
            let flush = suit_set.count_ones() == 1;
            // Check for straight.
            let straight: Option<u32> = find_straight(rank_set);

            match (straight, flush) {
                (None, false)                => HandRank::HighCard(rank_set),
                (None, true)                 => HandRank::Flush(rank_set),
                (Some(straight), false) => HandRank::Straight(straight),
                (Some(straight), true)  => HandRank::StraightFlush(straight),
            }
        },

        // 4 unique ranks => pair.
        4 => {
            let top_rank = count_to_rank[2];
            let minor_rank = rank_set ^ top_rank;
            HandRank::Pair(top_rank << 13 | minor_rank)
        },

        // 3 unique ranks => 3 of a kind or two pair.
        3 => {
            let n = count_to_rank[3];
            if n > 0 {
                let minor_rank = rank_set ^ n;
                HandRank::ThreeOfAKind(n << 13 | minor_rank)
            } else {
                let top_pair = count_to_rank[2];
                let minor_pair = rank_set ^ top_pair;
                HandRank::TwoPair(top_pair << 13 | minor_pair)
            }

        },

        // 2 unique ranks => full house or 4 of a kind.
        2 => {
            let n = count_to_rank[3];
            if n > 0 {
                let minor_rank = rank_set ^ n;
                HandRank::FullHouse(n << 13 | minor_rank)
            } else {
                let top_rank = count_to_rank[4];
                let minor_rank = rank_set ^ top_rank;
                HandRank::FourOfAKind(top_rank << 13 | minor_rank)
            }
        },
        
        // Only 4 cards of each rank => impossible.
        _ => unreachable!(),
    }
}

#[inline]
fn find_straight(rank_set: u32) -> Option<u32> {
    let left = rank_set &
        (rank_set << 1) &
        (rank_set << 2) &
        (rank_set << 3) &
        (rank_set << 4);
    let idx = left.leading_zeros();

    if idx < 32 {
        Some(32 - 4 - idx)
    } else if rank_set & 0b1_0000_0000_1111 == 0b1_0000_0000_1111 {
        Some(0)
    } else {
        None
    }
}

// Retain only the n most significant bits.
#[inline]
fn n_msb(r: u32, n: u32) -> u32 {
    let mut out = r;
    while out.count_ones() > n {
        out &= out - 1;
    }
    out
}

// Keep most significant bit.
#[inline]
fn msb(r: u32) -> u32 {
    1 << (31 - r.leading_zeros())
}
use std::{collections::HashMap, time};
use poker::{card::Card, deck::Deck};
use poker::evaluate::*;

#[test]
fn test_combo_5_senzee() {
    let deck = Deck::new();
    let cards = deck.into_iter().map(|card| card.bit_mask()).collect::<Vec<u32>>(); // convert to u32 for senzee's algo

    let mut rank_count: HashMap<HandRank, usize> = HashMap::new();
    let start = time::Instant::now();

    let mut hand = [0_u32; 5];
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
                        
                        let rank = rank_bit_mask_senzee(&hand).unwrap();
                        rank_count
                            .entry(HandRank::rank_variant(rank))
                            .and_modify(|count| *count += 1)
                            .or_insert(1);             
                    }
                }
            }
        }
    }

    println!("{:?}", start.elapsed());
    assert_eq!(*rank_count.get(&HandRank::HighCard(0)).unwrap(), 1302540);
    assert_eq!(*rank_count.get(&HandRank::Pair(0)).unwrap(), 1098240);
    assert_eq!(*rank_count.get(&HandRank::TwoPair(0)).unwrap(), 123552);
    assert_eq!(*rank_count.get(&HandRank::ThreeOfAKind(0)).unwrap(), 54912);
    assert_eq!(*rank_count.get(&HandRank::Straight(0)).unwrap(), 10200);
    assert_eq!(*rank_count.get(&HandRank::Flush(0)).unwrap(), 5108);
    assert_eq!(*rank_count.get(&HandRank::FullHouse(0)).unwrap(), 3744);
    assert_eq!(*rank_count.get(&HandRank::FourOfAKind(0)).unwrap(), 624);
    assert_eq!(*rank_count.get(&HandRank::StraightFlush(0)).unwrap(), 40);
}


#[test]
fn test_combo_5_two_plus_two() {
    
    let lookup_table = load_lookup_table("./data/lookup_table.bin").unwrap();
    let cards = Deck::new();
    
    let mut rank_count: HashMap<HandRank, usize> = HashMap::new();
    let mut hand = [Card::default(); 5];
    
    let start = time::Instant::now();
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
                        
                        let rank_num = rank_hand_5(&hand, &lookup_table);
                        let rank = HandRank::from(rank_num);
                        rank_count
                            .entry(HandRank::rank_variant(rank))
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                }
            }
        }
    }

    println!("{:?}", start.elapsed());
    assert_eq!(*rank_count.get(&HandRank::HighCard(0)).unwrap(), 1302540);
    assert_eq!(*rank_count.get(&HandRank::Pair(0)).unwrap(), 1098240);
    assert_eq!(*rank_count.get(&HandRank::TwoPair(0)).unwrap(), 123552);
    assert_eq!(*rank_count.get(&HandRank::ThreeOfAKind(0)).unwrap(), 54912);
    assert_eq!(*rank_count.get(&HandRank::Straight(0)).unwrap(), 10200);
    assert_eq!(*rank_count.get(&HandRank::Flush(0)).unwrap(), 5108);
    assert_eq!(*rank_count.get(&HandRank::FullHouse(0)).unwrap(), 3744);
    assert_eq!(*rank_count.get(&HandRank::FourOfAKind(0)).unwrap(), 624);
    assert_eq!(*rank_count.get(&HandRank::StraightFlush(0)).unwrap(), 40);
}

#[test]
fn test_combo_5_naive() {

    let cards = Deck::new();

    let mut rank_count: HashMap<HandRank, usize> = HashMap::new();
    let start = std::time::Instant::now();
    
    for a in 0..52 {
        for b in (a + 1)..52 {
            for c in (b + 1)..52 {
                for d in (c + 1)..52 {
                    for e in (d + 1)..52 {

                        let hand = [
                            cards[a],
                            cards[b],
                            cards[c],
                            cards[d],
                            cards[e],
                        ];

                        let rank = rank_cards_naive(&hand);
                        rank_count
                            .entry(HandRank::rank_variant(rank.unwrap()))
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                }
            }
        }
    }

    println!("Elapsed: {:?}", start.elapsed());
    assert_eq!(*rank_count.get(&HandRank::HighCard(0)).unwrap(), 1302540);
    assert_eq!(*rank_count.get(&HandRank::Pair(0)).unwrap(), 1098240);
    assert_eq!(*rank_count.get(&HandRank::TwoPair(0)).unwrap(), 123552);
    assert_eq!(*rank_count.get(&HandRank::ThreeOfAKind(0)).unwrap(), 54912);
    assert_eq!(*rank_count.get(&HandRank::Straight(0)).unwrap(), 10200);
    assert_eq!(*rank_count.get(&HandRank::Flush(0)).unwrap(), 5108);
    assert_eq!(*rank_count.get(&HandRank::FullHouse(0)).unwrap(), 3744);
    assert_eq!(*rank_count.get(&HandRank::FourOfAKind(0)).unwrap(), 624);
    assert_eq!(*rank_count.get(&HandRank::StraightFlush(0)).unwrap(), 40);
}

use std::{collections::HashMap, time};
use poker::{card::Card, deck::Deck};
use poker::evaluate::*;

#[test]
fn test_combo_7_senzee() {

    let deck = Deck::new();
    let cards = deck.into_iter().map(|card| card.bit_mask()).collect::<Vec<u32>>();
    let mut rank_count: HashMap<HandRank, usize> = HashMap::new();
    let mut hand = [0_u32; 7];
    let start = time::Instant::now();
    
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
                                
                                let rank = rank_bit_mask_senzee(&hand);
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

    println!("{:?}", start.elapsed());
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

#[test]
fn test_combo_7_two_plus_two() {

    let lookup_table = load_lookup_table().unwrap();
    let cards = Deck::new(); 
    
    let mut rank_count: HashMap<HandRank, usize> = HashMap::new();
    let mut hand = [Card::default(); 7];
    let start = time::Instant::now();

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
                                
                                let rank_num = rank_hand_7(&hand, &lookup_table);
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
        }
    }

    println!("{:?}", start.elapsed());
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


#[test]
fn test_combo_7_naive() {
     
    let cards = Deck::new(); 
    
    let mut rank_count: HashMap<HandRank, usize> = HashMap::new();
    let mut hand = [Card::default(); 7];
    let start = time::Instant::now();
    
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

                                let rank = rank_cards_naive(&hand);

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

    println!("{:?}", start.elapsed());
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
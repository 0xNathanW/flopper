use std::collections::HashMap;
use std::time;
use poker::{card::Deck, hand::HandRank};
use poker::evaluate::{
    rank_bit_mask_senzee, 
    rank_idx_two_plus_two,
    load_lookup_table, rank_cards_naive, 
};

#[test]
fn test_combo_5_senzee() {
    let deck = Deck::new();
    let cards = deck.into_iter().map(|card| card.bit_mask()).collect::<Vec<u32>>();
    
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
    
    let lookup_table = load_lookup_table().unwrap();
    let cards = Deck::new().into_iter().map(|c| c.idx()).collect::<Vec<usize>>(); 
    
    let mut rank_count: HashMap<HandRank, usize> = HashMap::new();
    let mut hand = [0_usize; 5];
    
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
                            .entry(HandRank::rank_variant(rank))
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

#[test]
fn test_eq() {

    use poker::card::Card;

    let lookup_table = load_lookup_table().unwrap();

    let cards = Deck::new();
    let mut cards_idx = cards.clone().into_iter().map(|c| c.idx()).collect::<Vec<usize>>();
    cards_idx.reverse();

    let mut hand = [Card::default(); 5];
    let mut hand2 = [0_usize; 5];

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

                        hand2[0] = cards_idx[a];
                        hand2[1] = cards_idx[b];
                        hand2[2] = cards_idx[c];
                        hand2[3] = cards_idx[d];
                        hand2[4] = cards_idx[e];

                        let rank = rank_cards_naive(&hand);
                        let rank2 = rank_idx_two_plus_two(&hand2, &lookup_table);

                        let naive_class = HandRank::rank_variant(rank);
                        let two_plus_two_class = HandRank::rank_variant(rank2);
                        
                        if naive_class != two_plus_two_class {
                            println!("{:?} != {:?}", naive_class, two_plus_two_class);
                            println!("naive: {:?}", hand);
                            println!("two_plus_two: {:?}", hand2.iter().map(|&c| Card::from_idx(c)).collect::<Vec<Card>>());
                        }
                    }
                }
            }
        }
    }
}
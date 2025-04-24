use crate::card::Card;
use super::{
    HandRank,
    bits::rank_hand_bits,
    senzee::rank_hand_senzee,
    two_plus_two::{rank_hand_2p2, load_lookup_table},
};

fn cards_arr(hand_str: &str) -> [Card; 5] {
    let cards: Vec<Card> = hand_str.split_whitespace()
        .map(|s| Card::from_str(s).unwrap())
        .collect();
    let mut result = [Card::default(); 5];
    result.copy_from_slice(&cards[0..5]);
    result
}

trait HandRankExt {
    fn get_type(&self) -> HandRankType;
}

// Type of poker hand
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandRankType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

impl HandRankExt for HandRank {
    fn get_type(&self) -> HandRankType {
        match self {
            HandRank::HighCard(_) => HandRankType::HighCard,
            HandRank::Pair(_) => HandRankType::Pair,
            HandRank::TwoPair(_) => HandRankType::TwoPair,
            HandRank::ThreeOfAKind(_) => HandRankType::ThreeOfAKind,
            HandRank::Straight(_) => HandRankType::Straight,
            HandRank::Flush(_) => HandRankType::Flush,
            HandRank::FullHouse(_) => HandRankType::FullHouse,
            HandRank::FourOfAKind(_) => HandRankType::FourOfAKind,
            HandRank::StraightFlush(_) => HandRankType::StraightFlush,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct EvaluatorTester {
        lookup_table: Vec<i32>,
    }
    
    impl EvaluatorTester {
        fn new() -> Self {
            let lookup_table = load_lookup_table("./data/lookup_table.bin").unwrap();
            Self { lookup_table }
        }
        
        // Test that all algorithms recognize the hand type correctly
        fn test_hand_type(&self, hand_str: &str, expected_type: HandRankType) {
            let hand = cards_arr(hand_str);
            
            // Test each algorithm
            let naive_rank = rank_hand_bits(&hand).unwrap();
            let senzee_rank = rank_hand_senzee(&hand).unwrap();
            let tpp_rank = rank_hand_2p2(&hand, &self.lookup_table).unwrap();
            
            assert_eq!(naive_rank.get_type(), expected_type, "Naive algorithm failed on {}", hand_str);
            assert_eq!(senzee_rank.get_type(), expected_type, "Senzee algorithm failed on {}", hand_str);
            assert_eq!(tpp_rank.get_type(), expected_type, "Two Plus Two algorithm failed on {}", hand_str);
        }
        
        // Test that all algorithms rank hands correctly relative to each other
        fn test_hand_comparison(&self, better_hand: &str, worse_hand: &str) {
            
            let hand1 = cards_arr(better_hand);
            let hand2 = cards_arr(worse_hand);
            
            // Test with all algorithms
            let naive1 = rank_hand_bits(&hand1).unwrap();
            let naive2 = rank_hand_bits(&hand2).unwrap();
            
            let senzee1 = rank_hand_senzee(&hand1).unwrap();
            let senzee2 = rank_hand_senzee(&hand2).unwrap();
            
            let tpp1 = rank_hand_2p2(&hand1, &self.lookup_table).unwrap();
            let tpp2 = rank_hand_2p2(&hand2, &self.lookup_table).unwrap();
            
            // Check that all algorithms agree on the ranking
            assert!(naive1 > naive2, "Naive: {:?} should be better than {:?}", better_hand, worse_hand);
            assert!(senzee1 > senzee2, "Senzee: {:?} should be better than {:?}", better_hand, worse_hand);
            assert!(tpp1 > tpp2, "Two Plus Two: {:?} should be better than {:?}", better_hand, worse_hand);
        }
    }
    
    #[test]
    fn test_straight_flush() {
        let tester = EvaluatorTester::new();
    
        tester.test_hand_type("As Ks Qs Js Ts", HandRankType::StraightFlush);
        tester.test_hand_type("5h 4h 3h 2h Ah", HandRankType::StraightFlush);
        
        tester.test_hand_comparison("As Ks Qs Js Ts", "5h 4h 3h 2h Ah");
    }
    
    #[test]
    fn test_four_of_a_kind() {
        let tester = EvaluatorTester::new();
        
        tester.test_hand_type("As Ad Ah Ac Ks", HandRankType::FourOfAKind);
        tester.test_hand_type("2s 2d 2h 2c As", HandRankType::FourOfAKind);
        
        tester.test_hand_comparison("As Ad Ah Ac Ks", "2s 2d 2h 2c As");
    }
    
    #[test]
    fn test_full_house() {
        let tester = EvaluatorTester::new();
        
        tester.test_hand_type("As Ad Ah Ks Kd", HandRankType::FullHouse);
        tester.test_hand_type("2s 2d 2h As Ad", HandRankType::FullHouse);
        
        tester.test_hand_comparison("As Ad Ah Ks Kd", "2s 2d 2h As Ad");
        tester.test_hand_comparison("2s 2d 2h Ks Kd", "2s 2d 2h 3s 3d");
    }
    
    #[test]
    fn test_flush() {
        let tester = EvaluatorTester::new();
        
        tester.test_hand_type("As Qs Ts 8s 5s", HandRankType::Flush);
        tester.test_hand_type("Kc Jc 9c 7c 4c", HandRankType::Flush);
        
        tester.test_hand_comparison("As Qs Ts 8s 5s", "Kc Jc 9c 7c 4c");
    }
    
    #[test]
    fn test_straight() {
        let tester = EvaluatorTester::new();
        
        tester.test_hand_type("As Kd Qh Jc Ts", HandRankType::Straight);
        tester.test_hand_type("5s 4d 3h 2c As", HandRankType::Straight);
        
        tester.test_hand_comparison("As Kd Qh Jc Ts", "5s 4d 3h 2c As");
    }
    
    #[test]
    fn test_three_of_a_kind() {
        let tester = EvaluatorTester::new();
        
        tester.test_hand_type("As Ad Ah Ks Qs", HandRankType::ThreeOfAKind);
        tester.test_hand_type("2s 2d 2h As Ks", HandRankType::ThreeOfAKind);
        
        tester.test_hand_comparison("As Ad Ah Ks Qs", "2s 2d 2h As Ks");
    }
    
    #[test]
    fn test_two_pair() {
        let tester = EvaluatorTester::new();
        
        tester.test_hand_type("As Ad Ks Kd Qs", HandRankType::TwoPair);
        tester.test_hand_type("2s 2d 3s 3d As", HandRankType::TwoPair);
        
        tester.test_hand_comparison("As Ad Ks Kd Qs", "2s 2d 3s 3d As");
    }
    
    #[test]
    fn test_one_pair() {
        let tester = EvaluatorTester::new();
        
        tester.test_hand_type("As Ad Ks Qs Js", HandRankType::Pair);
        tester.test_hand_type("2s 2d As Ks Qs", HandRankType::Pair);
        
        tester.test_hand_comparison("As Ad Ks Qs Js", "2s 2d As Ks Qs");
    }
    
    #[test]
    fn test_high_card() {
        let tester = EvaluatorTester::new();
        
        tester.test_hand_type("As Qd Th 8c 6s", HandRankType::HighCard);
        tester.test_hand_type("Ks Jd 9h 7c 5s", HandRankType::HighCard);
        
        tester.test_hand_comparison("As Qd Th 8c 6s", "Ks Jd 9h 7c 5s");
    }
    
    #[test]
    fn test_edge_cases() {
        let tester = EvaluatorTester::new();
        
        // Unsorted hand still detected correctly
        tester.test_hand_type("Ts As Js Ks Qs", HandRankType::StraightFlush);
        
        // Test kickers matter
        tester.test_hand_comparison("As Ad Kh Qs Js", "As Ad Kh Qs Ts");
        tester.test_hand_comparison("As Ks Qs Js 9s", "As Ks Qs Js 8s");
    }
}

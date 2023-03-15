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
    rank_cards_two_plus_two,
    rank_idx_two_plus_two,
    load_lookup_table, 
    save_lookup_table,
    generate_lookup_table
};

// Seperate tests for two_plus_two because it requires a lookup table.
#[cfg(test)]
#[allow(dead_code, unused_variables, irrefutable_let_patterns)]
mod test {
    use super::*;
    use crate::card::Card;
    use crate::hand::{Hand, HandRank};
    use naive::rank_hand_naive;

    // Check if a value is a variant of an enum.
    #[allow(irrefutable_let_patterns)]
    macro_rules! is_enum_variant {
        ($v:expr, $p:pat) => (
            if let $p = $v { true } else { false }
        );
    }

    fn board_from_str(board: &str) -> Vec<Card> {
        let mut cards = vec![];
        for card in board.split_whitespace() {
            cards.push(Card::from_str(card));
        }
        cards
    }

    // Hands are ordered by rank, ie idx i > idx j if hands[i] > hands[j].
    // Idx is the idx where important cards are for 5 card evaluation.
    fn test_evaluators(hands: Vec<Hand>, board: &[Card], expected: HandRank) {

        test_evaluator(&hands, board, expected, rank_hand_naive);
        test_evaluator(&hands, board, expected, rank_hand_senzee);
    }

    fn test_evaluator(hands: &Vec<Hand>, board: &[Card], expected: HandRank, evaluator: fn(&Hand, &[Card]) -> HandRank) {
        
        let rankings = hands.iter().map(|hand| evaluator(hand, board)).collect::<Vec<_>>();
        let mut last = HandRank::StraightFlush(u32::MAX); // Best hand possible.
        
        // Hands inputted are ordered by rank.
        rankings.iter().for_each(|rank| {
            assert!(is_enum_variant!(rank, expected));
            assert!(rank <= &last);
            last = *rank;
        });
    }


    #[test]
    fn test_high_card() {
        
        // 7 cards.
        let hands = vec![
            Hand::from_str("Ah Kd").unwrap(), // Ace high.
            Hand::from_str("Qd 9h").unwrap(), // Queen high.
            Hand::from_str("7c 4d").unwrap(), // Seven high.
        ];
        let board = board_from_str("8s Jc 5c 3h 2d");
        test_evaluators(hands, &board, HandRank::HighCard(0));

        // 6 cards.
        let hands = vec![
            Hand::from_str("Kd 9h").unwrap(),  // King hicker.
            Hand::from_str("Qd 9h").unwrap(),  // Queen kicker.
            Hand::from_str("Tc, 4d").unwrap(), // Ten kicker.
        ];
        let board = board_from_str("8s 2c 5c Ah");
        test_evaluators(hands, &board, HandRank::HighCard(0));
        
        // 5 cards.
        let hands = vec![
            Hand::from_str("Qd Jh").unwrap(), // Queen high.
            Hand::from_str("Qh 5h").unwrap() // Queen high with lower kicker.
        ];
        let board = board_from_str("8s 2c 3s");
        test_evaluators(hands, &board, HandRank::HighCard(0));
    }

    #[test]
    fn test_pair() {

        // 7 cards.
        let hands = vec![
            Hand::from_str("8d 8h").unwrap(), // 88
            Hand::from_str("Ac 7d").unwrap(), // 77 A kicker
            Hand::from_str("Qd 7d").unwrap(), // 77 Q kicker
            Hand::from_str("Qd 2h").unwrap(), // 22
        ];
        let board = board_from_str("2s Jc 7c 3h 4d");
        test_evaluators(hands, &board, HandRank::Pair(0));
        
        // 5 cards.
        let hands = vec![
            Hand::from_str("Kd 3h").unwrap(), // KK
            Hand::from_str("9d 3h").unwrap(), // 99
        ];
        let board = board_from_str("Kh 4c 9s");
        test_evaluators(hands, &board, HandRank::Pair(0));
    }

    #[test]
    fn test_two_pair() {

        // 6 cards.
        let hands = vec![
            Hand::from_str("Ac 6h").unwrap(), // AA 66
            Hand::from_str("5c 6s").unwrap(), // 66 55
        ];
        let board = board_from_str("As 6h 5h 3c");
        test_evaluators(hands, &board, HandRank::TwoPair(0));

        // 5 cards.
        let hands = vec![
            Hand::from_str("Ac 5d").unwrap(),
            Hand::from_str("7c 5s").unwrap(),
        ];
        let board = board_from_str("7h 5c Ah");
        test_evaluators(hands, &board, HandRank::TwoPair(0));
    }

    #[test]
    fn test_three_of_a_kind() {
        let hands = vec![
            Hand::from_str("Ac Ah").unwrap(), // AAA
            Hand::from_str("5c 6s").unwrap(), // 666
        ];
        let board = board_from_str("As 6c 6h 3c 2d");
        test_evaluators(hands, &board, HandRank::ThreeOfAKind(0));
    }

    #[test]
    fn test_four_of_a_kind() {
        let hands = vec![
            Hand::from_str("7c 7h").unwrap(), // 7777
        ];
        let board = board_from_str("7s 7d 6h 3c 2d");
        test_evaluators(hands, &board, HandRank::FourOfAKind(0));
    }

    #[test]
    fn test_full_house() {

        // 7 cards.
        let hands = vec![
            Hand::from_str("Ac Ah").unwrap(), // AAA 66         
        ];
        let board = board_from_str("As 6c 6h 3c 2d");
        test_evaluators(hands, &board, HandRank::FullHouse(0));

        // 5 cards.
        let hands = vec![
            Hand::from_str("3c 9h").unwrap(), // 999 33
        ];
        let board = board_from_str("3h 3s 9d");
        test_evaluators(hands, &board, HandRank::FullHouse(0));
    }

    #[test]
    fn test_straight() {
        // 7 cards.
        let hands = vec![
            Hand::from_str("9c 8d").unwrap(),
            Hand::from_str("8c 4d").unwrap(),
            Hand::from_str("4c 3d").unwrap(),
        ];
        let board = board_from_str("7s 6c 5h Kc 9d");
        test_evaluators(hands, &board, HandRank::Straight(0));
    }

    #[test]
    fn test_straight_flush() {
        // 5 card royal flush.
        let hands = vec![
            Hand::from_str("Ac Kc").unwrap(),
        ];
        let board = board_from_str("Qc Jc Tc");
        test_evaluators(hands, &board, HandRank::StraightFlush(0));
    }
}
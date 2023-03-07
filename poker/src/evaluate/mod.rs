
pub mod bits;
pub mod twoplustwo;

#[cfg(test)]
#[allow(dead_code, unused_variables, irrefutable_let_patterns)]
mod test {
    use super::*;
    use crate::card::Card;
    use crate::hand::{Hand, HandRank};
    use bits::{bits_hand_rank, bits_hand_rank_5};

    // Check if a value is a variant of an enum.
    #[allow(irrefutable_let_patterns)]
    macro_rules! is_enum_variant {
        ($v:expr, $p:pat) => (
            if let $p = $v { true } else { false }
        );
    }

    fn board_from_str(board: &str) -> [Card; 5] {
        let mut cards = [Card::default(); 5];
        for (i, card) in board.split_whitespace().enumerate() {
            cards[i] = card.try_into().unwrap();
        }
        cards
    }

    // Hands are ordered by rank, ie idx i > idx j if hands[i] > hands[j].
    // Idx is the idx where important cards are for 5 card evaluation.
    fn test_evaluators(hands: Vec<Hand>, board: &[Card], expected: HandRank, idx: usize) {

        test_evaluator(&hands, board, expected, bits_hand_rank);
        test_evaluator(&hands, &board[idx..idx+3], expected, bits_hand_rank_5);
    }

    fn test_evaluator(hands: &Vec<Hand>, board: &[Card], expected: HandRank, evaluator: fn(&Hand, &[Card]) -> HandRank) {
        
        let rankings = hands.iter().map(|hand| evaluator(hand, board)).collect::<Vec<_>>();
        let mut last = HandRank::StraightFlush(u32::MAX); // Best hand possible.
        
        rankings.iter().for_each(|rank| {
            assert!(is_enum_variant!(rank, expected));
            assert!(rank <= &last);
            last = *rank;
        });
    }


    #[test]
    fn test_high_card() {
        let hands = vec![
            Hand::from_str("Ah Kd").unwrap(), // Ace high.
            Hand::from_str("Qd 9h").unwrap(), // Queen high.
        ];
        let board = board_from_str("2s Jc 5c 3h 4d");
        test_evaluators(hands, &board, HandRank::HighCard(0), 0);
    }

    #[test]
    fn test_pair() {
        let hands = vec![
            Hand::from_str("8d 8h").unwrap(), // 88
            Hand::from_str("Ac 7d").unwrap(), // 77 A kicker
            Hand::from_str("Qd 7d").unwrap(), // 77 Q kicker
            Hand::from_str("Qd 2h").unwrap(), // 22
        ];
        let board = board_from_str("2s Jc 7c 3h 4d");
        test_evaluators(hands, &board, HandRank::Pair(0), 0);
    }

    #[test]
    fn test_two_pair() {
        let hands = vec![
            Hand::from_str("Ac 6h").unwrap(), // AA 66
            Hand::from_str("5c 6s").unwrap(), // 66 55
        ];
        let board = board_from_str("As 6h 5h 3c 2d");
        test_evaluators(hands, &board, HandRank::TwoPair(0), 0);
    }

    #[test]
    fn test_three_of_a_kind() {
        let hands = vec![
            Hand::from_str("Ac Ah").unwrap(), // AAA
            Hand::from_str("5c 6s").unwrap(), // 666
        ];
        let board = board_from_str("As 6c 6h 3c 2d");
        test_evaluators(hands, &board, HandRank::ThreeOfAKind(0), 0);
    }

    #[test]
    fn test_four_of_a_kind() {
        let hands = vec![
            Hand::from_str("7c 7h").unwrap(), // 7777
        ];
        let board = board_from_str("7s 7d 6h 3c 2d");
        test_evaluators(hands, &board, HandRank::FourOfAKind(0), 0);
    }

    #[test]
    fn test_full_house() {
        let hands = vec![
            Hand::from_str("Ac Ah").unwrap(), // AAA 66         
        ];
        let board = board_from_str("As 6c 6h 3c 2d");
        test_evaluators(hands, &board, HandRank::FullHouse(0), 0);
    }

    #[test]
    fn test_straight() {
        let hands = vec![
            Hand::from_str("9c 8d").unwrap(),
            Hand::from_str("8c 4d").unwrap(),
            Hand::from_str("4c 3d").unwrap(),
        ];
        let board = board_from_str("7s 6c 5h Kc 9d");
        test_evaluators(hands, &board, HandRank::Straight(0), 0);
    }
}
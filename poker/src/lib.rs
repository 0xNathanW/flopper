
pub mod card;
pub mod hand;
pub mod evaluate;
pub mod equity;
pub mod range;

pub fn board_from_str(s: &str) -> Result<Vec<card::Card>, hand::HandParseError> {
    let re = regex::Regex::new(r"([2-9TJQKA][cdhs])").unwrap();
    let mut cards = Vec::new();
    for cap in re.find_iter(s) {
        cards.push(cap.as_str().into());
    }
    Ok(cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_from_str() {
        // 3 card flop.
        let board = board_from_str("Ks2h4d").unwrap();
        assert_eq!(board.len(), 3);
        assert_eq!(board[0], card::Card::new(card::Rank::King, card::Suit::Spades));
        assert_eq!(board[1], card::Card::new(card::Rank::Two, card::Suit::Hearts));
        assert_eq!(board[2], card::Card::new(card::Rank::Four, card::Suit::Diamonds));
        // 5 card board.
        let board = board_from_str("As 2h Qd 5c 6s").unwrap();
        assert_eq!(board.len(), 5);
        assert_eq!(board[0], card::Card::new(card::Rank::Ace, card::Suit::Spades));
        assert_eq!(board[1], card::Card::new(card::Rank::Two, card::Suit::Hearts));
        assert_eq!(board[2], card::Card::new(card::Rank::Queen, card::Suit::Diamonds));
        assert_eq!(board[3], card::Card::new(card::Rank::Five, card::Suit::Clubs));
        assert_eq!(board[4], card::Card::new(card::Rank::Six, card::Suit::Spades));
    }
}
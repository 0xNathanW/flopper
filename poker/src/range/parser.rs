use thiserror::Error;
use crate::card::{Rank, CardParseError};
use super::{Range, pair_idxs, suited_idxs, offsuit_idxs};

// TODO: Add tests.

#[derive(Error, Debug)]
pub enum RangeParseError {

    #[error("Error parsing card: {0}")]
    CardParseError(#[from] CardParseError),
    
    #[error("No information on suitedness of hand: {0}{1}")]
    NoSuitedness(Rank, Rank),
    
    #[error("Unexpected token: {0}, expected: {1}")]
    UnexpectedToken(char, String),
    
    #[error("Invalid weight: {0}, must be between 0 and 1")]
    InvalidWeight(f32),
    
    #[error("Failed to parse weight: {0}")]
    WeightParseError(#[from] std::num::ParseFloatError),

    #[error("Empty range")]
    EmptyRange,
    
    #[error("Unexpected EOF")]
    UnexpectedEOF,
    
    #[error("{0}")]
    Custom(String),
}

impl Range {

    pub fn from_str(input: &str) -> Result<Range, RangeParseError> {

        let mut range = Range {
            name: input.to_string(),
            hands: [false; 1326],
        };

        if input.len() == 0 {
            return Err(RangeParseError::EmptyRange);
        }

        for elem in input.split(",").map(|s| s.trim()) {
            let mut chars = elem.chars();
    
            match elem.len() {
    
                // Single Pair
                2 => {
                    let rank_1 = Rank::from_str(chars.next().unwrap())?;
                    let rank_2 = Rank::from_str(chars.next().unwrap())?;
                    if rank_1 != rank_2 {
                        return Err(RangeParseError::NoSuitedness(rank_1, rank_2));
                    }
    
                    for idx in pair_idxs(rank_1) {
                        range.hands[idx] = true;
                    }
                },
    
                // Pair+, Suited, Offsuit
                3 => {
                    let rank_1 = Rank::from_str(chars.next().unwrap())?;
                    let rank_2 = Rank::from_str(chars.next().unwrap())?;
    
                    // Pair+
                    if rank_1 == rank_2 {
                        let p = chars.next().unwrap();
                        if p == '+' {
                            for rank in rank_1 as u8..=Rank::Ace as u8 {
                                for idx in pair_idxs(rank.into()) {
                                    range.hands[idx] = true;
                                }
                            }
                        } else {
                            return Err(RangeParseError::UnexpectedToken(p, "+".to_string()));
                        }
                    } 
    
                    // Suited, Offsuit
                    else {
                        let suitedness = chars.next().unwrap();
                        match suitedness {
                            's' => {
                                for idx in suited_idxs(rank_1, rank_2) {
                                    range.hands[idx] = true;
                                }
                            },
                            'o' => {
                                for idx in offsuit_idxs(rank_1, rank_2) {
                                    range.hands[idx] = true;
                                }
                            },
                            _ => return Err(RangeParseError::UnexpectedToken(suitedness, "s for suited or o for offsuit".to_string())),
                        }
                    }
                },
    
                // Suited+, Offsuit+
                4 => {
                    let rank_1 = Rank::from_str(chars.next().unwrap())?;
                    let rank_2 = Rank::from_str(chars.next().unwrap())?;
                    let max_rank = rank_1.max(rank_2);
                    let min_rank = rank_1.min(rank_2);
                    let suitedness = chars.next().unwrap();
    
                    if chars.next().unwrap() != '+' {
                        return Err(RangeParseError::UnexpectedToken('+', "+".to_string()));
                    }
    
                    match suitedness {
                        's' => {
                            for rank in min_rank as u8..max_rank as u8 {
                                for idx in suited_idxs(max_rank, rank.into()) {
                                    range.hands[idx] = true;
                                }
                            }
                        },
                        'o' => {
                            for rank in min_rank as u8..max_rank as u8 {
                                for idx in offsuit_idxs(max_rank, rank.into()) {
                                    range.hands[idx] = true;
                                }
                            }
                        },
                        _ => return Err(RangeParseError::UnexpectedToken(suitedness, "s for suited or o for offsuit".to_string())),
                    }
                },
    
                // Pair - Pair
                5 => {
                    let rank_1 = Rank::from_str(chars.next().unwrap())?;
                    let rank_2 = Rank::from_str(chars.next().unwrap())?;
    
                    if chars.next().unwrap() != '-' {
                        return Err(RangeParseError::UnexpectedToken('-', "-".to_string()));
                    }
    
                    let rank_3 = Rank::from_str(chars.next().unwrap())?;
                    let rank_4 = Rank::from_str(chars.next().unwrap())?;
    
                    if rank_1 != rank_2 || rank_3 != rank_4 {
                        return Err(RangeParseError::NoSuitedness(rank_1, rank_2));
                    }
    
                    let max_rank = rank_1.max(rank_3);
                    let min_rank = rank_1.min(rank_3);
    
                    for rank in min_rank as u8..=max_rank as u8 {
                        for idx in pair_idxs(rank.into()) {
                            range.hands[idx] = true;
                        }
                    }
                },
    
                // No input for 6 characters.
    
                // Suited - Suited, Offsuit - Offsuit
                7 => {
                    let rank_1 = Rank::from_str(chars.next().unwrap())?;
                    let rank_2 = Rank::from_str(chars.next().unwrap())?;
                    let max_rank_1 = rank_1.max(rank_2);
                    let min_rank_1 = rank_1.min(rank_2);
                    let suitedness_1 = chars.next().unwrap();
    
                    if chars.next().unwrap() != '-' {
                        return Err(RangeParseError::UnexpectedToken('-', "-".to_string()));
                    }
    
                    let rank_3 = Rank::from_str(chars.next().unwrap())?;
                    let rank_4 = Rank::from_str(chars.next().unwrap())?;
                    let max_rank_2 = rank_3.max(rank_4);
                    let min_rank_2 = rank_3.min(rank_4);
                    let suitedness_2 = chars.next().unwrap();
    
                    if suitedness_1 != suitedness_2 {
                        return Err(RangeParseError::UnexpectedToken(suitedness_2, "s for suited or o for offsuit".to_string()));
                    }
    
                    if max_rank_1 != max_rank_2 {
                        return Err(RangeParseError::UnexpectedToken(
                            format!("{}", max_rank_2).chars().next().unwrap(), 
                            format!("{}", max_rank_1).to_string())
                        );
                    }
    
                    if min_rank_1 > min_rank_2 {
                        return Err(RangeParseError::Custom("Min rank 1 is greater than min rank 2".to_string()));
                    }
    
                    match suitedness_1 {
                        's' => {
                            for rank in min_rank_1 as u8..=min_rank_2 as u8 {
                                for idx in suited_idxs(max_rank_1, rank.into()) {
                                    range.hands[idx] = true;
                                }
                            }
                        },
                        'o' => {
                            for rank in min_rank_1 as u8..=min_rank_2 as u8 {
                                for idx in offsuit_idxs(max_rank_1, rank.into()) {
                                    range.hands[idx] = true;
                                }
                            }
                        },
                        _ => return Err(RangeParseError::UnexpectedToken(suitedness_1, "s for suited or o for offsuit".to_string())),
                    }
                },
    
                _ => panic!("Unexpected number of characters in range input: {}", input),
            }
        }
    
        
        Ok(range)
    }
}


#[cfg(test)]
mod tests {
    use crate::range::Range;

    #[test]
    fn test_parser_error() {
        
        let cases = vec![
            "",
        ];

        for t in cases {
            let result = Range::from_str(t);
            assert!(result.is_err());
        }
    }
}
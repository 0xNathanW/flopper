use thiserror::Error;
use std::collections::BTreeSet;
use crate::card::{Rank, CardParseError};
use super::{Range, RangeHand};

#[derive(Error, Debug)]
pub enum RangeParseError {
    #[error("Error parsing card: {0}")]
    CardParseError(#[from] CardParseError),
    #[error("No information on suitedness of hand: {0}{1}")]
    NoSuitedness(Rank, Rank),
    #[error("Unexpected token: {0}, expected: {1}")]
    UnexpectedToken(char, String),
    #[error("Invalid percent: {0}, must be between 0 and 100")]
    InvalidPercent(f32),
    #[error("{0}")]
    Custom(String),
}

impl Range {

    pub fn from_str(input: &str) -> Result<Range, RangeParseError> {

        let mut hands = BTreeSet::new();

        for elem in input.split(",") {
            let elem = elem.trim();
            let mut chars = elem.chars();

            match elem.len() {

                // Single Pair
                2 => {
                    let rank_1 = Rank::from_str(chars.next().unwrap())?;
                    let rank_2 = Rank::from_str(chars.next().unwrap())?;
                    if rank_1 != rank_2 {
                        return Err(RangeParseError::NoSuitedness(rank_1, rank_2));
                    }
                    hands.insert(RangeHand::Pair(rank_1));
                },

                // Pair+, Suited, Offsuit
                3 => {
                    let rank_1 = Rank::from_str(chars.next().unwrap())?;
                    let rank_2 = Rank::from_str(chars.next().unwrap())?;

                    if rank_1 == rank_2 {
                        let p = chars.next().unwrap();
                        if p == '+' {
                            for rank in rank_1 as u8..=Rank::Ace as u8 {
                                hands.insert(RangeHand::Pair(rank.into()));
                            }
                        } else {
                            return Err(RangeParseError::UnexpectedToken(p, "+".to_string()));
                        }
                    } else {

                        let suitedness = chars.next().unwrap();
                        let max_rank = rank_1.max(rank_2);
                        let min_rank = rank_1.min(rank_2);

                        match suitedness {
                            's' => hands.insert(RangeHand::Suited(max_rank, min_rank)),
                            'o' => hands.insert(RangeHand::Offsuit(max_rank, min_rank)),
                            _ => return Err(RangeParseError::UnexpectedToken(suitedness, "s for suited or o for offsuit".to_string())),
                        };
                    }
                },

                // Suited+, Offsuit+
                4 => {

                    let rank_1 = Rank::from_str(chars.next().unwrap())?;
                    let rank_2 = Rank::from_str(chars.next().unwrap())?;
                    let max_rank = rank_1.max(rank_2);
                    let min_rank = rank_1.min(rank_2);
                    let suitedness = chars.next().unwrap();
                    
                    let p = chars.next().unwrap();
                    if p != '+' {
                        return Err(RangeParseError::UnexpectedToken(p, "+".to_string()));
                    }

                    match suitedness {
                        's' => {
                            for rank in min_rank as u8..=max_rank as u8 {
                                hands.insert(RangeHand::Suited(max_rank, rank.into()));
                            }
                        },
                        'o' => {
                            for rank in min_rank as u8..=max_rank as u8 {
                                hands.insert(RangeHand::Offsuit(max_rank, rank.into()));
                            }
                        },
                        _ => return Err(RangeParseError::UnexpectedToken(suitedness, "s for suited or o for offsuit".to_string())),
                    };
                },

                // Pair - Pair
                5 => {

                    let rank_1 = Rank::from_str(chars.next().unwrap())?;
                    let rank_2 = Rank::from_str(chars.next().unwrap())?;
                    
                    let p = chars.next().unwrap();
                    if p != '-' {
                        return Err(RangeParseError::UnexpectedToken(p, "-".to_string()));
                    }

                    let rank_3 = Rank::from_str(chars.next().unwrap())?;
                    let rank_4 = Rank::from_str(chars.next().unwrap())?;
                    
                    if rank_1 != rank_2 || rank_3 != rank_4 {
                        return Err(RangeParseError::NoSuitedness(rank_1, rank_2));
                    }
                    
                    let max_rank = rank_1.max(rank_3);
                    let min_rank = rank_1.min(rank_3);
                    
                    for rank in min_rank as u8..=max_rank as u8 {
                        hands.insert(RangeHand::Pair(rank.into()));
                    }
                },
                
                // No inputs for 6 characters.
                
                // Suited - Suited, Offsuit - Offsuit 
                7 => {
                    let rank_1 = Rank::from_str(chars.next().unwrap())?;
                    let rank_2 = Rank::from_str(chars.next().unwrap())?;
                    let max_rank_1 = rank_1.max(rank_2);
                    let min_rank_1 = rank_1.min(rank_2);
                    let suitedness_1 = chars.next().unwrap();
                    
                    let p = chars.next().unwrap();
                    if p != '-' {
                        return Err(RangeParseError::UnexpectedToken(p, "-".to_string()));
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
                            for rank in min_rank_1 as u8 ..= min_rank_2 as u8 {
                                hands.insert(RangeHand::Suited(max_rank_1, rank.into()));
                            }
                        },
                        'o' => {
                            for rank in min_rank_1 as u8 ..= min_rank_2 as u8 {
                                hands.insert(RangeHand::Offsuit(max_rank_1, rank.into()));
                            }
                        },
                        _ => return Err(RangeParseError::UnexpectedToken(suitedness_1, "s for suited or o for offsuit".to_string())),
                    };
                },
                
                _ => panic!("Unexpected number of characters in range input: {}", input),
            }
        }

        let mut total_combos = 0;
        let mut combo_counts = Vec::new();

        for hand in hands.iter() {
            match hand {
                RangeHand::Pair(_) => {
                    total_combos += 6;
                },
                RangeHand::Suited(_, _) => {
                    total_combos += 4;
                },
                RangeHand::Offsuit(_, _) => {
                    total_combos += 12;
                },
            }

            combo_counts.push((total_combos, hand.clone()));
        }

        Ok(Range {
            name: input.to_string(),
            hands,
            total_combos,
            combo_counts,
        })
    }
}

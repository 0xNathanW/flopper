use thiserror::Error;
use crate::card::{Rank, CardParseError};
use super::Range;
use super::range::{pair_idxs, suited_idxs, offsuit_idxs};

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
    
    #[error("Unexpected EOF")]
    UnexpectedEOF,
    
    #[error("{0}")]
    Custom(String),
}

impl Range {

    pub fn from_str(input: &str) -> Result<Range, RangeParseError> {

        let mut range = Range {
            name: input.to_string(),
            hands: [0.0; 1326],
        };

        if input.starts_with("[") {
            let mut chars = input.chars();
            // Weighted Range
            loop {
                // Parse Weight
                if let Some(c) = chars.next() {

                    if c == ' ' {
                        continue;
                    } else if c != '[' {
                        return Err(RangeParseError::UnexpectedToken(c, "[".to_string()));
                    }

                    let mut weight_str_pre = String::new();
                    // Parse Weight
                    loop {
                        let c = chars.next().ok_or(RangeParseError::UnexpectedEOF)?;
                        if c == ']' {
                            break;
                        }
                        weight_str_pre.push(c);
                    }
                    println!("weight_str_pre: {}", weight_str_pre);

                    let weight = str::parse::<f32>(&weight_str_pre.as_str())?;
                    if weight < 0.0 || weight > 1.0 {
                        return Err(RangeParseError::InvalidWeight(weight));
                    }

                    // Get range elements.
                    let mut r = String::new();
                    loop {
                        let c = chars.next().ok_or(RangeParseError::UnexpectedEOF)?;
                        if c == '[' {
                            break;
                        }
                        r.push(c);
                    }

                    if chars.next().ok_or(RangeParseError::UnexpectedEOF)? != '/' {
                        return Err(RangeParseError::UnexpectedToken(c, "/".to_string()));
                    }

                    let mut weight_str_suf = String::new();
                    // Parse weight closer
                    loop {
                        let c = chars.next().ok_or(RangeParseError::UnexpectedEOF)?;
                        if c == ']' {
                            break;
                        }
                        weight_str_suf.push(c);
                    }

                    if weight_str_pre != weight_str_suf {
                        return Err(RangeParseError::Custom("Weight open and close are not equal".to_string()));
                    }

                    parse_weight(&mut range, r.as_str(), weight)?;

                } else {
                    break;
                }
            }

        } else {
            // Unweighted Range
            parse_weight(&mut range, input, 1.0)?;
        }
        
        Ok(range)
    }
}

fn parse_weight(range: &mut Range, input: &str, weight: f32) -> Result<(), RangeParseError> {

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
                    range.hands[idx] = weight;
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
                                range.hands[idx] = weight;
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
                                range.hands[idx] = weight;
                            }
                        },
                        'o' => {
                            for idx in offsuit_idxs(rank_1, rank_2) {
                                range.hands[idx] = weight;
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
                                range.hands[idx] = weight;
                            }
                        }
                    },
                    'o' => {
                        for rank in min_rank as u8..max_rank as u8 {
                            for idx in offsuit_idxs(max_rank, rank.into()) {
                                range.hands[idx] = weight;
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
                        range.hands[idx] = weight;
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
                                range.hands[idx] = weight;
                            }
                        }
                    },
                    'o' => {
                        for rank in min_rank_1 as u8..=min_rank_2 as u8 {
                            for idx in offsuit_idxs(max_rank_1, rank.into()) {
                                range.hands[idx] = weight;
                            }
                        }
                    },
                    _ => return Err(RangeParseError::UnexpectedToken(suitedness_1, "s for suited or o for offsuit".to_string())),
                }
            },

            _ => panic!("Unexpected number of characters in range input: {}", input),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::Range;

    #[test]
    fn test_parser() {
        let range = Range::from_str("[0.5]22+[/0.5][1.0]K2s+[/1.0][0.7]A7o+[/0.7]").unwrap();
        println!("{:?}", range);
    }
}
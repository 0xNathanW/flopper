use std::fmt::Display;
use thiserror::Error;

// Bet sizing of the two players.
#[derive(Debug, Default, Clone)]
pub struct BetSizings {
    pub flop:  [BetSizingsStreet; 2],
    pub turn:  [BetSizingsStreet; 2],
    pub river: [BetSizingsStreet; 2],
}

// Contains available bet sizings for a specific street.
#[derive(Debug, Clone, PartialEq)]
pub struct BetSizingsStreet {
    // Bet sizings.
    pub bet: Vec<BetSize>,
    // Raise sizings.
    pub raise: Vec<BetSize>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum BetSize {
    // Bet a fixed amount.
    Absolute(i32),
    // Bet proportional to the pot - Eg. 50%.
    PotScaled(f64),
    // Bet proportional to previous bet (thus only for raises) - Eg. 2x.
    PrevScaled(f64),
    // Geometrically sized bets for .0 streets, max pot scaled size of .1.
    Geometric(i32, f64),
    // Bet whole stack.
    #[default]
    AllIn,
}

#[derive(Debug, Error)]
pub enum BetParseError {

    #[error("Empty bet string")]
    EmptyBetString,

    #[error("Cannot bet negative amount: {0}")]
    NegativeBetSize(f64),
    
    #[error("Invalid float: {0}")]
    FloatParseError(#[from] std::num::ParseFloatError),
    
    #[error("Invalid integer: {0}")]
    IntParseError(#[from] std::num::ParseIntError),

    #[error("Invalid suffix: {0}.  Must be one of: c, %, x")]
    InvalidSuffix(String),

    #[error("{0}")]
    Custom(String),
}

impl BetSizingsStreet {

    pub fn from_str(bet_str: &str, raise_str: &str) -> Result<BetSizingsStreet, BetParseError> {
            
            let bet = parse_bets(bet_str, false)?;
            let raise = parse_bets(raise_str, true)?;
    
            Ok(BetSizingsStreet { bet, raise })
    }
}

fn parse_bets(s: &str, raise: bool) -> Result<Vec<BetSize>, BetParseError> {

   let bets = s.split(",")
        .map(|x| x.trim().to_lowercase() )
        .filter(|x| !x.is_empty() )
        .map(|s| {

            if s == "allin" || s == "a" {
                Ok(BetSize::AllIn)
            
            // Geometric.
            } else if s.contains('e') {

                let mut split = s.split('e');
                let a = split.next().unwrap();
                let b = split.next().unwrap();

                let street = if a.is_empty() {
                    0
                } else {
                    let n = a.parse::<i32>()?;
                    if n == 0 || n > 100 {
                        return Err(BetParseError::Custom(
                            "Invalid geometric bet street number.".to_string()
                        ));
                    }
                    n
                };

                let max_pot_scaled = if b.is_empty() {
                    f64::INFINITY
                } else {
                    let s = b.strip_suffix('%').ok_or(BetParseError::InvalidSuffix(b.to_string()))?;
                    let f = s.parse::<f64>()?;
                    f / 100.0
                };

                Ok(BetSize::Geometric(street, max_pot_scaled))

            } else {

                match s.chars().last().unwrap() {
                    
                    'c' => {
                        let s = s.trim_end_matches('c');
                        let i = s.parse::<i32>()?;
                        Ok(BetSize::Absolute(i))
                    }

                    '%' => {
                        let s = s.trim_end_matches('%');
                        let f = s.parse::<f64>()?;
                        if f < 0.0 {
                            Err(BetParseError::NegativeBetSize(f))
                        } else {
                            Ok(BetSize::PotScaled(f / 100.0))
                        }
                    },

                    'x' => {
                        if !raise {
                            return Err(BetParseError::Custom(
                                "Can only scale previous bet on raises.".to_string()
                            ));                            
                        }

                        let s = s.trim_end_matches('x');
                        let f = s.parse::<f64>()?;
                        if f < 0.0 {
                            Err(BetParseError::NegativeBetSize(f))
                        } else {
                            Ok(BetSize::PrevScaled(f))
                        }
                    },

                    _ => Err(BetParseError::InvalidSuffix(s)),
                }
            }

        }).collect::<Result<Vec<BetSize>, BetParseError>>()?;

        // if bets.len() == 0 {
            // return Err(BetParseError::EmptyBetString);
        // } else {
            Ok(bets)
        // }
}

impl Display for BetSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetSize::Absolute(n) => write!(f, "Absolute: {}", n),
            BetSize::PotScaled(p) => write!(f, "Pot Scaled: {:.2}%", p * 100.0),
            BetSize::PrevScaled(p) => write!(f, "Prev Scaled: {:.2}x", p),
            BetSize::Geometric(s, p) => write!(f, "Geometric: {}e{:.2}%", s, p * 100.0),
            BetSize::AllIn => write!(f, "All In"),
        }
    }
}

impl Default for BetSizingsStreet {
    fn default() -> Self {
        let d = vec![BetSize::AllIn, BetSize::PotScaled(0.33), BetSize::PotScaled(0.5), BetSize::PotScaled(0.75), BetSize::PotScaled(1.0)];
        BetSizingsStreet {
            bet:   d.clone(),
            raise: d.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bets() {

        let bets = "allin, 150c , 50%, e";
        let raises = "a, 10C , 70%, 2x, 2e200%";

        let b = BetSizingsStreet::from_str(bets, raises).unwrap();
        let expected = BetSizingsStreet {
            bet: vec![
                BetSize::AllIn,
                BetSize::Absolute(150),
                BetSize::PotScaled(0.5),
                BetSize::Geometric(0, f64::INFINITY),
            ],
            raise: vec![
                BetSize::AllIn,
                BetSize::Absolute(10),
                BetSize::PotScaled(0.7),
                BetSize::PrevScaled(2.0),
                BetSize::Geometric(2, 2.0),
            ],
        };

        assert_eq!(b, expected);
    }

    #[test]
    fn test_parse_bet_err() {

        let t = ["", "0e", "E%", "c", "x"];

        for test in t {
            assert!(parse_bets(test, true).is_err());
        }
    }
}
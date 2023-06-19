use std::fmt::Display;

use thiserror::Error;

pub mod player {
    pub const PLAYER_OOP: u8            = 0b0000_0000;
    pub const PLAYER_IP: u8             = 0b0000_0001;
    pub const PLAYER_CHANCE: u8         = 0b0000_0010;
    pub const PLAYER_MASK: u8           = 0b0000_0011;
    pub const PLAYER_CHANCE_FLAG: u8    = 0b0000_0100;
    pub const PLAYER_TERMINAL_FLAG: u8  = 0b0000_1000;
    pub const PLAYER_FOLD_FLAG: u8      = 0b0001_1000;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Street {
    #[default]
    Flop,
    Turn,
    River,
}

impl From<usize> for Street {
    fn from(street: usize) -> Self {
        match street {
            0 => Street::Flop,
            1 => Street::Turn,
            2 => Street::River,
            _ => panic!("Invalid street."),
        }
    }
}

impl Display for Street {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Street::Flop  => write!(f, "Flop"),
            Street::Turn  => write!(f, "Turn"),
            Street::River => write!(f, "River"),
        }
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Input for {0} is invalid. {1}")]
    InvalidInput(String, String),

    #[error("Invalid terminal node found in action tree.")]
    InvalidTerminalNode,

    #[error("Solver requires at least cards for flop.")]
    MissingFlop,

    #[error("Sreets in config and on board do not match.")]
    MismatchedStreets,

    #[error("Game tree contains too many nodes.")]
    TooManyNodes,
}

pub mod action;
pub mod node;
pub mod game;
pub mod postflop;
pub mod latch;
pub mod solve;
pub mod exploitability;
pub mod cfv;
pub mod slice_ops;

mod bets;
mod tree;
mod build_data;
mod config;

// Describes actions a player can take at a decision point.
#[derive(Clone, Copy, Default, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Action {
    #[default]
    None,
    Fold,
    Check,
    Call,
    Bet(u32),
    Raise(u32),
    AllIn(u32),
    Chance(u8),
}

pub use bets::{BetSizings, BetSizingsStreet, BetSize};
pub use tree::{ActionTree, ActionTreeNode};
pub use config::TreeConfig;
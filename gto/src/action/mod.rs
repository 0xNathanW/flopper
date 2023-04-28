
mod bets;
mod tree;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Street {
    #[default]
    Flop,
    Turn,
    River,
}

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
    Chance(poker::card::Card),
}

pub use bets::{BetSizings, StreetBetSizings};
pub use tree::{ActionTree, TreeConfig};
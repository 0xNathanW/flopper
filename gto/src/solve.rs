use std::mem::MaybeUninit;
use crate::{game::Game, node::GameNode, latch::Latch};
use crate::exploitability::*;

// Performs CFR+ algorithm, returning exploitability of computed strategy.
pub fn solve<G: Game>(game: &mut G, max_iters: u32, target_exploitablility: f32) -> f32 {

    if game.solved() || !game.ready() {
        panic!("Game not ready or already solved");
    }

    let mut root = game.root();
    let mut exploitability = compute_exploitability(game);

    dbg!(exploitability);
    0.0
}
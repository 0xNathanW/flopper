use std::mem::MaybeUninit;
use crate::{latch::LatchGuard, node::GameNode};

pub trait Game: Send + Sync {

    type Node: GameNode;

    fn root(&self) -> LatchGuard<Self::Node>;

    fn num_private_hands(&self, player: usize) -> usize;

    fn intial_weights(&self, player: usize) -> &[f32];

    fn evaluate(
        &self,
        node: &Self::Node,
        player: usize,
        result: &mut [MaybeUninit<f32>],
        cf_reach: &[f32],
    );

    // Effective number of chances.
    fn chance_factor(&self, node: &Self::Node) -> usize;

    fn solved(&self) -> bool;

    fn set_solved(&mut self);

    fn ready(&self) -> bool {
        true
    }

    fn raked(&self) -> bool {
        false
    }

    fn isomorphic_chances(&self, _node: &Self::Node) -> &[u8] {
        unreachable!()
    }

    fn isomorphic_swap(&self, _node: &Self::Node, _idx: usize) -> &[Vec<(u16, u16)>; 2] {
        unreachable!()
    }

    fn locking_strategy(&self, _node: &Self::Node) -> &[f32] {
        unreachable!()
    }

    fn compression_enabled(&self) -> bool {
        false
    }
}


use crate::{game::Game, latch::LatchGuard, Street};
use super::{PostFlopNode, PostFlopGame, ProcessState};

impl Game for PostFlopGame {

    type Node = PostFlopNode;
    
    fn root(&self) -> LatchGuard<Self::Node> {
        self.node_arena[0].lock()
    }

    fn num_private_hands(&self, player: usize) -> usize {
        self.hands[player].len()
    }

    fn intial_weights(&self, player: usize) -> &[f32] {
        &self.initial_weights[player]
    }

    fn evaluate(
        &self, 
        node: &Self::Node, 
        player: usize, 
        result: &mut [std::mem::MaybeUninit<f32>], 
        cf_reach: &[f32]
    ) {
        if self.bunching_num_dead == 0 {
            self.evaluate_no_bunching(result, node, player, cf_reach);
        } else {
            self.evaluate_bunching(result, node, player, cf_reach);
        }
    }

    fn chance_factor(&self, node: &Self::Node) -> usize {
        if !node.turn.is_dealt() {
            45 - self.bunching_num_dead
        } else {
            44 - self.bunching_num_dead
        }
    }

    fn solved(&self) -> bool {
        self.state == ProcessState::Solved
    }

    fn set_solved(&mut self) {
        self.state = ProcessState::Solved;
        let history = self.action_history.clone();
        self.apply_history(&history);
    }

    fn ready(&self) -> bool {
        self.state == ProcessState::MemoryAllocated && self.storage_mode == Street::River
    }

    fn raked(&self) -> bool {
        self.tree_config.rake > 0.0 && self.tree_config.rake_cap > 0.0
    }

    fn isomorphic_chances(&self, node: &Self::Node) -> &[u8] {
        if !self.board.is_turn_dealt() {
            &self.isomorphism_ref_turn
        } else {
            &self.isomorphism_ref_river[node.turn.0 as usize]
        }
    }

    fn isomorphic_swap(&self, node: &Self::Node, idx: usize) -> &[Vec<(u16, u16)>; 2] {
        if !self.board.is_turn_dealt() {
            &self.isomorphism_swap_turn[self.isomorphism_card_turn[idx] as usize & 3]
        } else {
            &self.isomorphism_swap_river[node.turn.0 as usize & 3]
            [self.isomorphism_card_river[node.turn.0 as usize &3][idx] as usize & 3]
        }
    }

    fn locking_strategy(&self, node: &Self::Node) -> &[f32] {
        if !node.locked {
            &[]
        } else {
            &self.locking_strat[&self.node_idx(node)]
        }
    }

    fn compression_enabled(&self) -> bool {
        self.compression_enabled
    }
}

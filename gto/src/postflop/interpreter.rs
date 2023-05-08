use poker::Hand;
use crate::{game::Game, latch::{LatchGuard, Latch}};
use super::{PostFlopGame, ProcessState, PostFlopNode};

impl PostFlopGame {

    pub fn to_root(&mut self) {
        
        if self.state <= ProcessState::Uninitialised {
            panic!("Game not initialised");
        }

        self.action_history.clear();
        self.node_history.clear();
        self.normalised_cache = false;
        self.turn = self.board.turn;
        self.river = self.board.river;
        self.turn_swap = None;
        self.river_swap = None;
        self.total_bet_amount = [0, 0];

        self.weights[0].copy_from_slice(&self.initial_weights[0]);
        self.weights[1].copy_from_slice(&self.initial_weights[1]);
        self.assign_zero_weights();
    }

    pub fn assign_zero_weights(&mut self) {
        if self.bunching_num_dead == 0 {
            
            let mut board_mask: u64 = 0;
            if self.turn.is_dealt() {
                board_mask |= 1 << self.turn.0;
            }
            if self.river.is_dealt() {
                board_mask |= 1 << self.river.0;
            }

            for player in 0..2 {
                
                let mut dead: u64 = (1 << 52) - 1;

                for &hand in &self.hands[player] {
                    let mask: u64 = (1 << hand.0.0) | (1 << hand.1.0);
                    if mask & board_mask == 0 {
                        dead &= mask;
                    }
                    if dead == 0 {
                        break;
                    }
                }

                dead |= board_mask;

                self.hands[player].iter()
                    .zip(self.weights[player].iter_mut())
                    .for_each(|(&hand, weight)| {
                        let mask: u64 = (1 << hand.0.0) | (1 << hand.1.0);
                        if mask & dead != 0 {
                            *weight = 0.0;
                        }
                    });
            }

        } else {
            for player in 0..2 {

                let node = self.current_node();
                let opp_len = self.num_private_hands(player ^ 1);

                let idxs = if !node.turn.is_dealt() {
                    &self.bunching_num_flop[player]
                } else if !node.river.is_dealt() {
                    &self.bunching_num_turn[player][node.turn.0 as usize]
                } else {
                    &self.bunching_num_river[player][Hand(node.turn, node.river).idx()]
                };
                drop(node);

                let mut weights_buf = Vec::new();
                let weights = if self.turn_swap.is_none() && self.river_swap.is_none() {
                    &mut self.weights[player]
                } else {
                    weights_buf.extend_from_slice(&self.weights[player]);
                    self.apply_swap(&mut weights_buf, player, true);
                    &mut weights_buf
                };

                for (w, &idx) in weights.iter_mut().zip(idxs.iter()) {
                    if idx == 0 {
                        *w = 0.0;
                    } else {
                        let s = &self.bunching_arena[idx..idx + opp_len];
                        if s.iter().all(|&n| n == 0.0) {
                            *w = 0.0;
                        }
                    }
                }

                if self.turn_swap.is_some() || self.river_swap.is_some() {
                    self.apply_swap(&mut weights_buf, player, false);
                    self.weights[player].copy_from_slice(&weights_buf);
                }
            }
        }
    } 

    pub fn apply_history(&mut self, history: &[usize]) {
        
        if self.state <= ProcessState::Uninitialised {
            panic!("Game not initialised");
        }

        self.to_root();
        for &action in history {
            // self.play(action);
        }
    }

    pub fn current_node(&self) -> LatchGuard<PostFlopNode> {
        self.node_arena[self.node_history.last().cloned().unwrap_or(0)].lock()
    }

    pub fn node_idx(&self, node: &PostFlopNode) -> usize {
        let ptr = node as *const _ as *const Latch<PostFlopNode>;
        unsafe {
            ptr.offset_from(self.node_arena.as_ptr()) as usize
        }
    }

    pub fn apply_swap(&self, s: &mut [f32], player: usize, reverse: bool) {

        let turn_swap = self.turn_swap
            .map(|suit| &self.isomorphism_swap_turn[suit as usize][player]);
        
        let river_swap = self.river_swap
            .map(|(turn_suit, suit)| &self.isomorphism_swap_river[turn_suit as usize][suit as usize][player]);
    
        let swaps = if !reverse {
            [turn_swap, river_swap]
        } else {
            [river_swap, turn_swap]
        };

        for swap in swaps.into_iter().flatten() {
            for &(i, j) in swap {
                s.swap(i as usize, j as usize);
            }
        }
    }
}
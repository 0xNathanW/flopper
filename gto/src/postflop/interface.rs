use poker::{Hand, Card};
use crate::{
    game::Game, 
    latch::{LatchGuard, Latch}, 
    node::GameNode, Street, 
    action::Action, 
    cfv::{normalised_stategy_compressed, normalised_strategy}, 
    slice_ops::*, player::PLAYER_IP
};
use super::{PostFlopGame, ProcessState, PostFlopNode};

impl PostFlopGame {

    // Play an action.
    pub fn play(&mut self, action: usize) {
        
        if self.state <= ProcessState::MemoryAllocated {
            panic!("Memory not allocated, can't play.");
        }

        if self.is_terminal_node() {
            panic!("Can't play on terminal node.");
        }

        if self.is_chance_node() {
            if self.storage_mode == Street::Flop || (self.board.is_turn_dealt() && self.storage_mode == Street::Turn) {
                panic!("Incompatable storage mdoe.");
            }

            let actual_card = if action == usize::MAX {
                self.possible_cards().trailing_zeros() as u8
            } else {
                action as u8
            };

            let action_card = if let Some((s1, s2)) = self.turn_swapped_suit {
                if actual_card & 3  == s1 {
                    actual_card - s1 + s2
                } else if actual_card & 3 == s2 {
                    actual_card - s2 + s1
                } else {
                    actual_card
                }
            } else {
                actual_card
            };
            
            let actions = self.available_actions();
            let mut action_idx = usize::MAX;

            for (i, &action) in actions.iter().enumerate() {
                if action == Action::Chance(action_card) {
                    action_idx = i;
                    break;
                }
            }

            if action_idx == usize::MAX {
                
                let node = self.node();
                let isomorphism = self.isomorphic_chances(&node);
                let isomorphic_cards = if !node.turn.is_dealt() {
                    &self.isomorphism_card_turn
                } else {
                    &self.isomorphism_card_river[node.turn.0 as usize & 3]
                };

                for (i, &repr_idx) in isomorphism.iter().enumerate() {
                    if action_card == isomorphic_cards[i] {
                        action_idx = repr_idx as usize;

                        if !self.turn.is_dealt() {
                            if let Action::Chance(repr_card) = actions[repr_idx as usize] {
                                self.turn_swapped_suit = Some((action_card & 3, repr_card & 3));
                            }
                            self.turn_swap = Some(action_card & 3);

                        } else {
                            self.river_swap = Some((
                                self.turn.0 & 3,
                                self.isomorphism_card_river[self.turn.0 as usize & 3][i] & 3,
                            ));
                        }
                        break;
                    }
                }
            }

            if action_idx == usize::MAX {
                panic!("Invalid action.");
            }

            let node_idx = self.node_idx(&self.node().play(action_idx));
            self.node_history.push(node_idx);
            if !self.turn.is_dealt() {
                self.turn = Card(actual_card);
            } else {
                self.river = Card(actual_card);
            }

            self.assign_zero_weights();
        
        } else {

            let node = self.node();
            if action >= node.num_actions() {
                panic!("Invalid action.");
            }

            let player = node.player();
            let num_hands = self.num_private_hands(player);

            if node.num_actions() > 1 {
                let strategy = self.strategy();
                let weights = row(&strategy, action, num_hands);
                mul_slice(&mut self.weights[player], weights);
            }

            // Cache counterfactual values.
            let node = self.node();
            let vec = if self.compression_enabled {
                let node = node;
                let slice = row(node.cf_values_compressed(), action, num_hands);
                let scale = node.cf_value_scale();
                decode_signed_slice(slice, scale)
            } else {
                row(node.cf_values(), action, num_hands).to_vec()
            };
            self.cf_values_cache[player].copy_from_slice(&vec);

            let node = self.node();
            match node.play(action).last_action {
                
                Action::Call => {
                    self.total_bet_amount[player] = self.total_bet_amount[player ^ 1];
                },

                Action::Bet(amount) | Action::Raise(amount) | Action::AllIn(amount) => {
                    let last_amount = match node.last_action {
                        Action::Bet(a) | Action::Raise(a) | Action::AllIn(a) => a,
                        _ => 0,
                    };
                    let to_call = self.total_bet_amount[player ^ 1] - self.total_bet_amount[player];
                    self.total_bet_amount[player] += amount - last_amount + to_call;
                },

                _ => {},
            }

            let node_idx = self.node_idx(&self.node().play(action));
            self.node_history.push(node_idx);
        }

        self.action_history.push(action);
        self.normalised_cache = false;
    }

    // Apply a history of actions.
    pub fn apply_history(&mut self, history: &[usize]) {    
        if self.state <= ProcessState::Uninitialised {
            panic!("Game not initialised");
        }
        // Back to root and play each action.
        self.to_root();
        for &action in history {
            self.play(action);
        }
    }

    pub fn history(&self) -> &[usize] {
        if self.state <= ProcessState::Uninitialised {
            panic!("Game not initialised");
        }
        &self.action_history
    }

    // Available actions at the current node.
    pub fn available_actions(&self) -> Vec<Action> {
        if self.state <= ProcessState::Uninitialised {
            panic!("Game not initialised");
        }

        if self.is_terminal_node() {
            return Vec::new();
        } else {
            self.node().children().iter().map(|c| c.lock().last_action).collect()
        }
    }

    // 0 = Out of position, 1 = In position.
    pub fn current_player(&self) -> usize {
        if self.state <= ProcessState::Uninitialised {
            panic!("Game not initialised");
        }
        self.node().player()
    }

    pub fn hands(&self, player: usize) -> &[Hand] {
        if self.state <= ProcessState::Uninitialised {
            panic!("Game not initialised");
        }
        &self.hands[player]
    }

    // Amount each player has bet.
    pub fn total_bet_amount(&self) -> [i32; 2] {
        self.total_bet_amount
    }

    pub fn current_board(&self) -> Vec<Card> {
        if self.state <= ProcessState::Uninitialised {
            panic!("Game not initialised");
        }
        self.board.as_vec()
    }

    // Returns LatchGuard to current node.
    pub fn node(&self) -> LatchGuard<PostFlopNode> {
        self.node_arena[self.node_history.last().cloned().unwrap_or(0)].lock()
    }

    // Idx of suppied node.
    pub fn node_idx(&self, node: &PostFlopNode) -> usize {
        let ptr = node as *const _ as *const Latch<PostFlopNode>;
        unsafe {
            ptr.offset_from(self.node_arena.as_ptr()) as usize
        }
    }

    // Whether the current node is a terminal node.
    pub fn is_terminal_node(&self) -> bool {
        if self.state <= ProcessState::Uninitialised {
            panic!("Game not initialised");
        }
        let node = self.node();
        node.is_terminal() || node.amount == self.tree_config.effective_stack
    }

    // Whether the current node is a chance node.
    pub fn is_chance_node(&self) -> bool {
        if self.state <= ProcessState::Uninitialised {
            panic!("Game not initialised");
        }
        self.node().is_chance() && !self.is_terminal_node()
    }

    pub fn equity(&self, player: usize) -> Vec<f32> {
        if self.state <= ProcessState::Uninitialised {
            panic!("Game not initialised");
        }
        if !self.normalised_cache {
            panic!("Normalised weights not cached.");
        }

        let num_hands = self.num_private_hands(player);
        let temp = if self.bunching_num_dead == 0 {
            let mut temp = vec![0.0; num_hands];

            if self.river.is_dealt() {
                self.equity_internal(&mut temp, player, self.turn.0, self.river.0, 0.5);

            } else if self.turn.is_dealt() {
                for r in 0..52 {
                    if self.turn.0 != r {
                        self.equity_internal(&mut temp, player, self.turn.0, r, 0.5 / 44.0);
                    }
                }
            
            } else {
                for t in 0..52 {
                    for r in (t + 1)..52 {
                        self.equity_internal(&mut temp, player, t, r, 1.0 / (45.0 * 44.0));
                    }
                }
            }

            temp.into_iter().map(|n| n as f32).collect()

        } else {
            let mut temp = self.equity_internal_bunching(player);
            self.apply_swap(&mut temp, player, false);
            temp
        };

        temp.iter()
            .zip(self.weights[player].iter())
            .zip(self.normalised_weights[player].iter())
            .map(|((&v, &w_raw), &w_normalised)| {
                if w_normalised > 0.0 {
                    v * (w_raw / w_normalised) + 0.5
                } else {
                    0.0
                }
            }).collect()
    }

    pub fn expected_values(&self, player: usize) -> Vec<f32> {
        if self.state != ProcessState::Solved {
            panic!("Game not solved");
        }
        if !self.normalised_cache {
            panic!("Normalised weights not cached.");
        }

        let expected_values_detail = self.expected_values_detail(player);

        if self.is_terminal_node() || self.is_chance_node() || self.current_player() != player {
            return expected_values_detail;
        }

        let num_actions = self.node().num_actions();
        let num_hands = self.num_private_hands(player);
        let strategy = self.strategy();

        let mut out = Vec::with_capacity(num_hands);
        for i in 0..num_hands {
            let mut expected_value = 0.0;
            for j in 0..num_actions {
                let idx = i + j * num_hands;
                expected_value += expected_values_detail[idx] * strategy[idx];
            }
            out.push(expected_value);
        }

        out
    }

    pub fn expected_values_detail(&self, player: usize) -> Vec<f32> {
        if self.state != ProcessState::Solved {
            panic!("Game not solved");
        }
        if !self.normalised_cache {
            panic!("Normalised weights not cached.");
        }

        let node = self.node();
        let num_hands = self.num_private_hands(player);

        let mut chance_factor = 1;
        if !self.board.is_turn_dealt() && self.turn.is_dealt() {
            chance_factor *= 45 - self.bunching_num_dead;
        }
        if !self.board.is_river_dealt() && self.river.is_dealt() {
            chance_factor *= 44 - self.bunching_num_dead;
        }

        let num_combos = match self.bunching_num_dead {
            0 => self.num_combos,
            _ => self.bunching_num_combos,
        };

        let mut have_actions = false;
        let mut normaliser = (num_combos * chance_factor as f64) as f32;

        let mut out = if node.is_terminal() {
            normaliser = num_combos as f32;

            let mut out = Vec::with_capacity(num_hands);
            let mut cf_reach = self.weights[player ^ 1].clone();
            
            self.apply_swap(&mut cf_reach, player ^ 1, true);
            self.evaluate(&node, player, out.spare_capacity_mut(), &cf_reach);
            
            unsafe { out.set_len(num_hands); }
            out

        } else if node.is_chance() && node.cf_values_storage_player() == Some(player) {

            if self.compression_enabled {
                let slice = node.cf_values_chance_compressed();
                let scale = node.cf_value_chance_scale();
                decode_signed_slice(slice, scale)
            } else {
                node.cf_values_chance().to_vec()
            }

        } else if node.has_ip_cf_values() && player == PLAYER_IP as usize {

            if self.compression_enabled {
                let slice = node.ip_cf_values_compressed();
                let scale = node.ip_cf_value_scale();
                decode_signed_slice(slice, scale)
            } else {
                node.ip_cf_values().to_vec()
            }
        
        } else if player == self.current_player() {

            have_actions = true;
            if self.compression_enabled {
                let slice = node.cf_values_compressed();
                let scale = node.cf_value_scale();
                decode_signed_slice(slice, scale)
            } else {
                node.cf_values().to_vec()
            }
        
        } else {
            self.cf_values_cache[player].to_vec()
        };

        let starting_pot = self.tree_config.starting_pot;
        let total_bet_amount = self.total_bet_amount();
        let bias = (total_bet_amount[player] - total_bet_amount[player ^ 1]).max(0);

        out.chunks_exact_mut(num_hands).enumerate().for_each(|(action, row)| {
            let is_fold = have_actions && self.node().play(action).last_action == Action::Fold;
            self.apply_swap(row, player, false);

            row.iter_mut()
                .zip(self.weights[player].iter())
                .zip(self.normalised_weights[player].iter())
                .for_each(|((v, &w_raw), &w_normalised)| {
                    if is_fold || w_normalised == 0.0 {
                        *v = 0.0;
                    } else {
                        *v *= normaliser * (w_raw / w_normalised);
                        *v += starting_pot as f32 * 0.5 + (self.node().amount + bias) as f32;
                    }
                });
        });

        out
    }

    // List of cards that could be dealt.
    pub fn possible_cards(&self) -> u64 {
        if self.state <= ProcessState::Uninitialised {
            panic!("Game not initialised");
        }

        if !self.is_chance_node() {
            return 0;
        }

        let mut flop_mask = self.board.flop_mask();
        let mut dead: u64 = 0;

        if self.bunching_num_dead == 0 {
            if self.board.is_turn_dealt() {
                dead |= self.board.turn.mask();
            }

            'outer: for c in 0..52 {
                let b = 1 << c;
                let new_mask = flop_mask | b;

                if new_mask != flop_mask {
                    for &oop_hand in &self.hands[0] {
                        
                        let oop_mask: u64 = oop_hand.mask();
                        if oop_mask & new_mask != 0 {
                            continue;
                        }
                        
                        let combined = oop_mask | new_mask;
                        for &ip_hand in &self.hands[1] {
                            let ip_mask = ip_hand.mask();
                            if ip_mask & combined == 0 {
                                continue 'outer;
                            }
                        }
                    }
                }

                dead |= b
            }
        
        } else {

            let turn = self.node().turn;
            if turn.is_dealt() {
                flop_mask |= turn.mask();
            }
            
            let ip_len = self.num_private_hands(1);
            let mut children = Vec::new();
            let (iso_ref, iso_card) = if !turn.is_dealt() {
                (&self.isomorphism_ref_turn, &self.isomorphism_card_turn)
            } else {
                (&self.isomorphism_ref_river[turn.0 as usize], &self.isomorphism_card_river[turn.0 as usize & 3])
            };

            'outer: for card in 0..52 {
                let b: u64 = 1 << card;
                let new_mask = flop_mask | b;

                if let Some(p) = iso_card.iter().position(|&c| c == card) {
                    let ref_card = children[iso_ref[p] as usize];
                    dead |= ((dead >> ref_card) & 1) << card;
                    continue;
                }

                if new_mask != flop_mask {
                    children.push(card);
                    let idxs = if !turn.is_dealt() {
                        &self.bunching_num_turn[0][card as usize]
                    } else {
                        &self.bunching_num_river[0][Hand(turn, Card(card)).idx()]
                    };

                    for &idx in idxs {
                        if idx == 0 {
                            continue;
                        }
                        let slice = &self.bunching_arena[idx..idx + ip_len];
                        if slice.iter().any(|&n| n > 0.0) {
                            continue 'outer;
                        }
                    }
                }

                dead |= b;
            }
            
            if let Some((s1, s2)) = self.turn_swapped_suit {
                let suit_mask: u64 = 0x1_1111_1111_1111;
                let mod_mask = (suit_mask << s1) | (suit_mask << s2);
                let swapped_1 = ((dead >> s1) & suit_mask) << s2;
                let swapped_2 = ((dead >> s2) & suit_mask) << s1;
                dead = (dead & !mod_mask) | swapped_1 | swapped_2;
            }
        }
        
        ((1 << 52) -1) ^ dead
    }

    // Makes the current node the root node.
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

    // Returns vec with length (num actions) * (num private hands)
    // Probability of ith action with jth hand is stored at [i * num_hands + j]
    pub fn strategy(&self) -> Vec<f32> {
        if self.state < ProcessState::MemoryAllocated {
            panic!("Memory not allocated");
        }
        if self.is_terminal_node() {
            panic!("Can't get strategy at terminal node.");
        }
        if self.is_chance_node() {
            panic!("Can't get strategy at chance node.");
        }

        let node = self.node();
        let player = self.current_player();
        let num_actions = node.num_actions();
        let num_hands = self.num_private_hands(player);

        let mut out = if self.compression_enabled {
            normalised_stategy_compressed(node.strategy_compressed(), num_actions)
        } else {
            normalised_strategy(node.strategy(), num_actions)
        };

        let locking = self.locking_strategy(&node);
        apply_locking_strategy(&mut out, locking);

        out.chunks_exact_mut(num_hands).for_each(|chunk| {
            self.apply_swap(chunk, player, false);
        });

        out
    }

    pub fn weights(&self, player: usize) -> &[f32] {
        if self.state <= ProcessState::Uninitialised {
            panic!("Game not initialised");
        }
        &self.weights[player]
    }

    pub fn normalised_weights(&self, player: usize) -> &[f32] {
        if self.state <= ProcessState::Uninitialised {
            panic!("Game not initialised");
        }
        if !self.normalised_cache {
            panic!("Normalised weights not cached.");
        }

        &self.normalised_weights[player]
    }

    pub fn cache_normalised_weights(&mut self) {
        if self.state <= ProcessState::Uninitialised {
            panic!("Game not initialised");
        }

        if self.normalised_cache {
            return;
        }

        // No bunching.
        if self.bunching_num_dead == 0 {
            
            let mut board_mask = 0_u64;
            if self.turn.is_dealt() {
                board_mask |= 1 << self.turn.0;
            }
            if self.river.is_dealt() {
                board_mask |= 1 << self.river.0;
            }
            
            let mut weight_sum = [0.0; 2];
            let mut weight_sum_minus = [[0.0; 52]; 2];

            for player in 0..2 {
                let weight_sum_player = &mut weight_sum[player];
                let weight_sum_minus_player = &mut weight_sum_minus[player];
                self.hands[player].iter().zip(self.weights[player].iter()).for_each(|(&hand, &weight)| {
                    if hand.mask() & board_mask == 0 {
                        let w = weight as f64;
                        *weight_sum_player += w;
                        weight_sum_minus_player[hand.0.0 as usize] += w;
                        weight_sum_minus_player[hand.1.0 as usize] += w;
                    }
                });
            }

            for player in 0..2 {

                let player_hands = &self.hands[player];
                let same_hand_idx = &self.same_hand_idx[player];
                let player_weights = &self.weights[player];
                let opp_weights = &self.weights[player ^ 1];
                let opp_weights_sum = weight_sum[player ^ 1];
                let opp_weights_sum_minus = &weight_sum_minus[player ^ 1];

                self.normalised_weights[player].iter_mut().enumerate().for_each(|(i, w)| {
                    let hand = player_hands[i];
                    if hand.mask() & board_mask == 0 {
                        
                        let same_idx = same_hand_idx[i];
                        let opp_weight_same = if same_idx == u16::MAX {
                            0.0
                        } else {
                            opp_weights[same_idx as usize] as f64
                        };

                        let opp_weight = opp_weights_sum + opp_weight_same
                            - opp_weights_sum_minus[hand.0.0 as usize]
                            - opp_weights_sum_minus[hand.1.0 as usize];
                        *w = player_weights[i] * opp_weight as f32;
                    } else {
                        *w = 0.0;
                    }
                });
            }
        
        } else {

            let mut weights_buf = [Vec::new(), Vec::new()];
            let weights = if self.turn_swap.is_none() && self.river_swap.is_none() {
                &self.weights
            } else {
                weights_buf[0].extend_from_slice(&self.weights[0]);
                weights_buf[1].extend_from_slice(&self.weights[1]);
                self.apply_swap(&mut weights_buf[0], 0, true);
                self.apply_swap(&mut weights_buf[1], 1, true);
                &weights_buf
            };

            for player in 0..2 {
                
                let node = self.node();
                let idxs = if node.river.is_dealt() {
                    &self.bunching_num_river[player][Hand(node.turn, node.river).idx()]
                } else if node.turn.is_dealt() {
                    &self.bunching_num_turn[player][node.turn.0 as usize]
                } else {
                    &self.bunching_num_flop[player]
                };

                let opp_len = self.num_private_hands(player ^ 1);
                let mut normalised_weights = idxs.iter().zip(weights[player].iter()).map(|(&idx, &w)| {
                    if idx != 0 {
                        let slice = &self.bunching_arena[idx..idx + opp_len];
                        w * inner_product(&weights[player ^ 1], slice)
                    } else {
                        0.0
                    }
                }).collect::<Vec<_>>();

                self.apply_swap(&mut normalised_weights, player, false);
                self.normalised_weights[player] = normalised_weights;
            }
        }

        self.normalised_cache = true;
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

                let node = self.node();
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

    fn equity_internal(
        &self, 
        result: &mut [f64],
        player: usize,
        turn: u8,
        river: u8,
        amount: f64,
    ) {

        let pair_idx = Hand(Card(turn), Card(river)).idx();
        let hand_strength = &self.hand_strengths[pair_idx];
        let player_strength = &hand_strength[player];
        let opp_strength = &hand_strength[player ^ 1];

        let player_len = player_strength.len();
        let opp_len = opp_strength.len();
        if player_len == 0 || opp_len == 0 {
            return;
        }

        let player_cards = &self.hands[player];
        let opp_cards = &self.hands[player ^ 1];

        let opp_weights = &self.weights[player ^ 1];
        let mut weight_sum = 0.0;
        let mut weight_minus = [0.0; 52];
        
        let valid_player_strength = &player_strength[1..player_len - 1];
        let mut i = 1;

        for &(stength, idx) in valid_player_strength {
            unsafe {
                while opp_strength.get_unchecked(i).0 < stength {
                    let opp_idx = opp_strength.get_unchecked(i).1 as usize;
                    let hand = opp_cards.get_unchecked(opp_idx);
                    let weight_idx = *opp_weights.get_unchecked(opp_idx) as f64;
                    weight_sum += weight_idx;
                    weight_minus[hand.0.0 as usize] += weight_idx;
                    weight_minus[hand.1.0 as usize] += weight_idx;
                    i += 1;
                }

                let hand = player_cards.get_unchecked(idx as usize);
                let opp_weight = weight_sum
                    - weight_minus[hand.0.0 as usize]
                    - weight_minus[hand.1.0 as usize];
                *result.get_unchecked_mut(idx as usize) += amount * opp_weight;
            }
        }

        weight_sum = 0.0;
        weight_minus.fill(0.0);
        i = opp_len - 2;

        for &(strength, idx) in valid_player_strength.iter().rev() {
            unsafe {
                while opp_strength.get_unchecked(i).0 > strength {
                    let opp_idx = opp_strength.get_unchecked(i).1 as usize;
                    let hand = opp_cards.get_unchecked(opp_idx);
                    let weight_idx = *opp_weights.get_unchecked(opp_idx) as f64;
                    weight_sum += weight_idx;
                    weight_minus[hand.0.0 as usize] += weight_idx;
                    weight_minus[hand.1.0 as usize] += weight_idx;
                    i -= 1;
                }

                let hand = player_cards.get_unchecked(idx as usize);
                let opp_weight = weight_sum
                    - weight_minus[hand.0.0 as usize]
                    - weight_minus[hand.1.0 as usize];
                *result.get_unchecked_mut(idx as usize) -= amount * opp_weight;
            }
        }
    }

    fn equity_internal_bunching(&self, player: usize) -> Vec<f32> {
        
        let mut weights_buf = Vec::new();
        let opp_weights = if self.turn_swap.is_none() && self.river_swap.is_none() {
            &self.weights[player ^ 1]
        } else {
            weights_buf.extend_from_slice(&self.weights[player ^ 1]);
            self.apply_swap(&mut weights_buf, player, true);
            &weights_buf
        };
        let node = self.node();
        let opp_len = opp_weights.len();

        if !node.river.is_dealt() {
            let idxs = if node.turn.is_dealt() {
                &self.bunching_coeff_turn[player][node.turn.0 as usize]
            } else {
                &self.bunching_coeff_flop[player]
            };
            
            idxs.iter().map(|&idx| {
                if idx != 0 {
                    let slice = &self.bunching_arena[idx..idx + opp_len];
                    0.5 * inner_product(opp_weights, slice)
                } else {
                    0.0
                }
            }).collect()
        
        } else {

            let pair_idx = Hand(node.turn, node.river).idx();
            let idxs = &self.bunching_num_river[player][pair_idx];
            let player_strength = &self.bunching_strength[pair_idx][player];
            let opp_strength = &self.bunching_strength[pair_idx][player ^ 1];

            idxs.iter().zip(player_strength).map(|(&idx, &strength)| {
                if idx != 0 {
                    inner_product_cond(
                        opp_weights,
                        &self.bunching_arena[idx..idx + opp_len],
                        opp_strength,
                        strength,
                        0.5,
                        -0.5,
                        0.0,
                    )
                } else {
                    0.0
                }
            }).collect()
        }
    }
}
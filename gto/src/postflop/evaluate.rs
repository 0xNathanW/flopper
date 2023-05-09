use std::mem::MaybeUninit;
use poker::hand::Hand;
use crate::{slice_ops::inner_product_cond, player::*};
use super::{PostFlopGame, PostFlopNode};

impl PostFlopGame {

    pub fn evaluate_no_bunching(
        &self,
        result: &mut [MaybeUninit<f32>],
        node: &PostFlopNode,
        player: usize,
        cf_reach: &[f32]
    ) {
        
        let pot = (self.tree_config.starting_pot + 2 * node.amount) as f64;
        let half_pot = pot / 2.0;
        let rake = self.tree_config.rake_cap.min(pot * self.tree_config.rake);
        let amount_win = (half_pot - rake) / self.num_combos;
        let amount_lose = -half_pot / self.num_combos;

        let player_cards = &self.hands[player];
        let opp_cards = &self.hands[player ^ 1];

        let mut cf_reach_sum = 0.0;
        let mut cf_reach_minus = [0.0; 52];

        result.iter_mut().for_each(|x| { x.write(0.0); });
        let result = unsafe{
            &mut *(result as *mut _ as *mut [f32])
        };

        // Fold occured.
        if node.player & PLAYER_FOLD_FLAG == PLAYER_FOLD_FLAG {
            
            let folded_player = node.player & PLAYER_MASK;
            let payoff = if folded_player as usize != player {
                amount_win
            } else {
                amount_lose
            };
            
            let valid_idxs = if node.river.is_dealt() {
                &self.valid_idxs_river[Hand(node.turn, node.river).idx()]
            } else if node.turn.is_dealt() {
                &self.valid_idxs_turn[node.turn.0 as usize]
            } else {
                &self.valid_idxs_flop  
            };

            // Iterate valid hands of opponent.
            for i in &valid_idxs[player ^ 1] {
                unsafe {
                    // Probability of reaching this hand, given a particular strategy.
                    let cf_reach_i = *cf_reach.get_unchecked(*i as usize);
                    if cf_reach_i != 0.0 {
                        let hand = *opp_cards.get_unchecked(*i as usize);
                        cf_reach_sum += cf_reach_i as f64;
                        *cf_reach_minus.get_unchecked_mut(hand.0.0 as usize) += cf_reach_i as f64;
                        *cf_reach_minus.get_unchecked_mut(hand.1.0 as usize) += cf_reach_i as f64;
                    }
                }
            }

            if cf_reach_sum == 0.0 {
                return;
            }

            let same_hand_idx = &self.same_hand_idx[player];
            // Now iterate valid hands of player.
            for i in &valid_idxs[player] {
                unsafe {
                    
                    let hand = *player_cards.get_unchecked(*i as usize);
                    let same_idx = *same_hand_idx.get_unchecked(*i as usize);
                    let cf_reach_same = if same_idx == u16::MAX {
                        0.0
                    } else {
                        *cf_reach.get_unchecked(same_idx as usize) as f64
                    };

                    let cf_reach = cf_reach_sum + cf_reach_same 
                        - *cf_reach_minus.get_unchecked(hand.0.0 as usize)
                        - *cf_reach_minus.get_unchecked(hand.1.0 as usize);
                    
                    *result.get_unchecked_mut(*i as usize) = (payoff * cf_reach) as f32;
                }
            }
        
        // Showdown optimised for no rake.
        } else if rake == 0.0 {

            let pair_idx = Hand(node.turn, node.river).idx();
            let hand_strength = &self.hand_strengths[pair_idx];
            let player_strength = &hand_strength[player];
            let opp_strength = &hand_strength[player ^ 1];
            let valid_player_strength = &player_strength[1..player_strength.len() - 1];       
            let mut i = 1;

            for &(strength, idx) in valid_player_strength {
                unsafe{
                    while opp_strength.get_unchecked(i).0 < strength {
                        
                        let opp_idx = opp_strength.get_unchecked(i).1 as usize;
                        let cf_reach_i = *cf_reach.get_unchecked(opp_idx);
                        
                        if cf_reach_i != 0.0 {
                            let hand = *opp_cards.get_unchecked(opp_idx);
                            cf_reach_sum += cf_reach_i as f64;
                            *cf_reach_minus.get_unchecked_mut(hand.0.0 as usize) += cf_reach_i as f64;
                            *cf_reach_minus.get_unchecked_mut(hand.1.0 as usize) += cf_reach_i as f64;
                        }

                        i += 1;
                    }
                    
                    let hand = *player_cards.get_unchecked(idx as usize);
                    let cf_reach = cf_reach_sum 
                        - cf_reach_minus.get_unchecked(hand.0.0 as usize)
                        - cf_reach_minus.get_unchecked(hand.1.0 as usize);
                    *result.get_unchecked_mut(idx as usize) = (amount_win * cf_reach) as f32;
                }
            }

            cf_reach_sum = 0.0;
            cf_reach_minus.fill(0.0);
            i = opp_strength.len() - 2;

            for &(strength, idx) in valid_player_strength.iter().rev() {
                unsafe {
                    while opp_strength.get_unchecked(i).0 > strength {
                    
                        let opp_idx = opp_strength.get_unchecked(i).1 as usize;
                        let cf_reach_i = *cf_reach.get_unchecked(opp_idx);
                        
                        if cf_reach_i != 0.0 {
                            let hand = *opp_cards.get_unchecked(opp_idx);
                            cf_reach_sum += cf_reach_i as f64;
                            *cf_reach_minus.get_unchecked_mut(hand.0.0 as usize) += cf_reach_i as f64;
                            *cf_reach_minus.get_unchecked_mut(hand.1.0 as usize) += cf_reach_i as f64;
                        }

                        i -= 1;
                    }
                    
                    let hand = *player_cards.get_unchecked(idx as usize);
                    let cf_reach = cf_reach_sum 
                        - cf_reach_minus.get_unchecked(hand.0.0 as usize)
                        - cf_reach_minus.get_unchecked(hand.1.0 as usize);
                    *result.get_unchecked_mut(idx as usize) += (amount_lose * cf_reach) as f32;
                }
            }
            
        // Showdown.
        } else {

            let amount_tie = -0.5 * rake / self.num_combos;
            let same_hand_idx = &self.same_hand_idx[player];

            let pair_idx = Hand(node.turn, node.river).idx();
            let hand_strength = &self.hand_strengths[pair_idx];
            let player_strength = &hand_strength[player];
            let opp_strength = &hand_strength[player ^ 1];

            let valid_player_strength = &player_strength[1..player_strength.len() - 1];
            let valid_opp_strength = &opp_strength[1..opp_strength.len() - 1];

            for &(_, idx) in valid_opp_strength {
                unsafe {
                    let cf_reach_i = *cf_reach.get_unchecked(idx as usize);
                    if cf_reach_i != 0.0 {
                        let hand = *opp_cards.get_unchecked(idx as usize);
                        cf_reach_sum += cf_reach_i as f64;
                        *cf_reach_minus.get_unchecked_mut(hand.0.0 as usize) += cf_reach_i as f64;
                        *cf_reach_minus.get_unchecked_mut(hand.1.0 as usize) += cf_reach_i as f64;
                    }
                }
            }

            if cf_reach_sum == 0.0 {
                return;
            }

            let mut cf_reach_sum_win = 0.0;
            let mut cf_reach_sum_tie = 0.0;
            let mut cf_reach_minus_win = [0.0; 52];
            let mut cf_reach_minus_tie = [0.0; 52];

            let mut i = 1; 
            let mut j = 1;
            let mut prev_strength = 0;

            for &(strength, idx) in valid_player_strength {
                unsafe {
                    if strength > prev_strength {
                        prev_strength = strength;

                        if i < j {
                            cf_reach_sum_win = cf_reach_sum_tie;
                            cf_reach_minus_win = cf_reach_minus_tie;
                            i = j;
                        }

                        while opp_strength.get_unchecked(i).0 < strength {
                            let opp_idx = opp_strength.get_unchecked(i).1 as usize;
                            let hand = *opp_cards.get_unchecked(opp_idx);
                            let cf_reach_i = *cf_reach.get_unchecked(opp_idx);
                            cf_reach_sum_win += cf_reach_i as f64;
                            *cf_reach_minus_win.get_unchecked_mut(hand.0.0 as usize) += cf_reach_i as f64;
                            *cf_reach_minus_win.get_unchecked_mut(hand.1.0 as usize) += cf_reach_i as f64;
                        }

                        if j < i {
                            cf_reach_sum_tie = cf_reach_sum_win;
                            cf_reach_minus_tie = cf_reach_minus_win;
                            j = i;
                        }

                        while opp_strength.get_unchecked(j).0 == strength {
                            let opp_idx = opp_strength.get_unchecked(j).1 as usize;
                            let hand = *opp_cards.get_unchecked(opp_idx);
                            let cf_reach_j = *cf_reach.get_unchecked(opp_idx);
                            cf_reach_sum_tie += cf_reach_j as f64;
                            *cf_reach_minus_tie.get_unchecked_mut(hand.0.0 as usize) += cf_reach_j as f64;
                            *cf_reach_minus_tie.get_unchecked_mut(hand.1.0 as usize) += cf_reach_j as f64;
                            j += 1;
                        }
                    }

                    let hand = *player_cards.get_unchecked(idx as usize);
                    
                    let cf_reach_total = cf_reach_sum 
                        - cf_reach_minus.get_unchecked(hand.0.0 as usize)
                        - cf_reach_minus.get_unchecked(hand.1.0 as usize);
                    
                    let cf_reach_win = cf_reach_sum_win
                        - cf_reach_minus_win.get_unchecked(hand.0.0 as usize)
                        - cf_reach_minus_win.get_unchecked(hand.1.0 as usize);

                    let cf_reach_tie = cf_reach_sum_tie
                        - cf_reach_minus_tie.get_unchecked(hand.0.0 as usize)
                        - cf_reach_minus_tie.get_unchecked(hand.1.0 as usize);
                    
                    let same_i = *same_hand_idx.get_unchecked(idx as usize);
                    let cf_reach_same = if same_i == u16::MAX {
                        0.0
                    } else {
                        *cf_reach.get_unchecked(same_i as usize) as f64
                    };

                    let cf_value = amount_win * cf_reach_win
                        + amount_tie * (cf_reach_tie - cf_reach_win + cf_reach_same) 
                        + amount_lose * (cf_reach_total - cf_reach_tie);

                    *result.get_unchecked_mut(idx as usize) = cf_value as f32;
                }
            }
        }
    }

    pub fn evaluate_bunching(
        &self,
        result: &mut [MaybeUninit<f32>],
        node: &PostFlopNode,
        player: usize,
        cf_reach: &[f32],
    ) {
        
        let pot = (self.tree_config.starting_pot + 2 * node.amount) as f64;
        let half_pot = pot / 2.0;
        let rake = (pot * self.tree_config.rake).min(self.tree_config.rake_cap);
        let amount_win = ((half_pot - rake) / self.bunching_num_combos) as f32;
        let amount_lose = (-half_pot / self.bunching_num_combos) as f32;
        let amount_tie = (-0.5 * rake / self.bunching_num_combos) as f32;
        let opp_len = self.hands[player ^ 1].len();

        // Fold.
        if node.player & PLAYER_FOLD_FLAG == PLAYER_FOLD_FLAG {
            
            let fold_player = node.player & PLAYER_MASK;
            let payoff = if fold_player as usize != player {
                amount_win
            } else {
                amount_lose
            };
            
            let idxs = if node.river.is_dealt() {
                &self.bunching_num_river[player][Hand(node.turn, node.river).idx()]
            } else if node.turn.is_dealt() {
                &self.bunching_num_turn[player][node.turn.0 as usize]
            } else {
                &self.bunching_num_flop[player]
            };

            result.iter_mut().zip(idxs).for_each(|(r, &idx)| {
                if idx != 0 {
                    let s = &self.bunching_arena[idx..idx + opp_len];
                    let inner_product = s.iter().zip(cf_reach).map(|(s, r)| s * r).sum::<f32>();
                    r.write(payoff * inner_product);
                } else {
                    r.write(0.0);
                }
            });
        
        // Showdown.
        } else {
            let pair_idx = Hand(node.turn, node.river).idx();
            let idxs = &self.bunching_num_river[player][pair_idx];
            let player_strength = &self.bunching_strength[pair_idx][player];
            let opp_strength = &self.bunching_strength[pair_idx][player ^ 1];

            result.iter_mut()
                .zip(idxs)
                .zip(player_strength)
                .for_each(|((r, &idx), &strength)| {
                    if idx != 0 {
                        r.write(inner_product_cond(
                            cf_reach,
                            &self.bunching_arena[idx..idx + opp_len],
                            opp_strength,
                            strength,
                            amount_win,
                            amount_lose,
                            amount_tie,
                        ));
                    } else {
                        r.write(0.0);
                    }
                })
        }
    }
}
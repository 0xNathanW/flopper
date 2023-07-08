use std::mem::MaybeUninit;
use crate::cfv::compute_cfv;
use crate::{game::Game, node::GameNode, latch::Latch};
use crate::{exploitability::*, slice_ops::*};

struct DiscountParams {
    alpha: f32,
    beta:  f32,
    gamma: f32,
}

impl DiscountParams {
    pub fn new(num_iter: u32) -> DiscountParams {

        let msb_even = match num_iter {
            0 => 0,
            i => 1 << ((i.leading_zeros() ^ 31) & !1),
        };

        let t_alpha = (num_iter as i32 - 1).max(0) as f64;
        let t_gamma = (num_iter - msb_even) as f64;

        let pow_alpha = t_alpha * t_alpha.sqrt();
        let pow_gamma = (t_gamma / (t_gamma + 1.0)).powi(3);

        DiscountParams {
            alpha: (pow_alpha / (pow_alpha + 1.0)) as f32,
            beta: 0.5,
            gamma: pow_gamma as f32,
        }
    }
}

// Performs CFR+ algorithm, returning exploitability of computed strategy.
pub fn solve<G: Game>(game: &mut G, max_iters: u32, target_exploitablility: f32) -> f32 {

    if game.solved() || !game.ready() {
        panic!("Game not ready or already solved");
    }

    let mut root = game.root();
    let mut exploitability = compute_exploitability(game);

    for i in 0..max_iters {
        
        if exploitability < target_exploitablility {
            break;
        }

        let params = DiscountParams::new(i);

        for player in 0..2 {
            let mut result = Vec::with_capacity(game.num_private_hands(player));
            solve_cfv(
                game,
                &mut root,
                result.spare_capacity_mut(),
                player,
                game.intial_weights(player ^ 1),
                &params,
            );
        }

        if (i + 1) % 10 == 0 || i + 1 == max_iters {
            exploitability = compute_exploitability(game);
        }
    }

    finalise(game);

    exploitability
}

// Runs a single iteration of CFR+ algorithm.
pub fn solve_step<G: Game>(game: &G, i: u32) {
    
    if game.solved() || !game.ready() {
        panic!("Game not ready or already solved");
    }

    let mut root = game.root();
    let params = DiscountParams::new(i);

    for player in 0..2 {
        let mut result = Vec::with_capacity(game.num_private_hands(player));
        solve_cfv(
            game,
            &mut root,
            result.spare_capacity_mut(),
            player,
            game.intial_weights(player ^ 1),
            &params,
        );
    }
}

fn solve_cfv<G: Game>(
    game: &G,
    node: &mut G::Node,
    result: &mut [MaybeUninit<f32>],
    player: usize,
    cf_reach: &[f32],
    params: &DiscountParams,
) {

    if node.is_terminal() {
        game.evaluate(node, player, result, cf_reach);
        return;
    }

    let num_actions = node.num_actions();
    let num_hands = result.len();

    if num_actions == 1 && !node.is_chance() {
        let child = &mut node.play(0);
        solve_cfv(game, child, result, player, cf_reach, params);
        return;
    }

    let cfv_actions = Latch::new(Vec::with_capacity(num_actions * num_hands));

    if node.is_chance() {
        
        let mut cf_reach_updated = Vec::with_capacity(cf_reach.len());
        mul_slice_scalar_uninit(
            cf_reach_updated.spare_capacity_mut(),
            cf_reach,
            1.0 / game.chance_factor(node) as f32,
        );
        unsafe { cf_reach_updated.set_len(cf_reach.len()); }

        node.child_op(|action| {
            solve_cfv(
                game,
                &mut node.play(action),
                row_mut(cfv_actions.lock().spare_capacity_mut(), action, num_hands),
                player,
                &cf_reach_updated,
                params,
            );
        });

        let mut result_f64 = Vec::with_capacity(num_hands);
        let mut cfv_actions = cfv_actions.lock();
        unsafe { cfv_actions.set_len(num_actions * num_hands) };
        sum_slices_uninit_f64(result_f64.spare_capacity_mut(), &cfv_actions);
        unsafe { result_f64.set_len(num_hands) };

        let isomorphic_chances = game.isomorphic_chances(node);

        for (i, &iso_idx) in isomorphic_chances.iter().enumerate() {
        
            let swap_list = &game.isomorphic_swap(node, i)[player];
            let temp = row_mut(&mut cfv_actions, iso_idx as usize, num_hands);
            apply_swap(temp, swap_list);

            result_f64.iter_mut().zip(&*temp).for_each(|(r, &v)| {
                *r += v as f64;
            });

            apply_swap(temp, swap_list);
        }

        result.iter_mut().zip(&result_f64).for_each(|(r, &v)| {
            r.write(v as f32);
        });
    
    } else if node.player() == player {

        node.child_op(|action| {
            solve_cfv(
                game,
                &mut node.play(action),
                row_mut(cfv_actions.lock().spare_capacity_mut(), action, num_hands),
                player,
                cf_reach,
                params,
            );
        });
        
        let mut strategy = if game.compression_enabled() {
            regret_matching_compressed(node.regrets_compressed(), num_actions)
        } else {
            regret_matching(node.regrets(), num_actions)
        };
        
        let locking = game.locking_strategy(node);
        apply_locking_strategy(&mut strategy, locking);
        
        let mut cfv_actions = cfv_actions.lock();
        unsafe { cfv_actions.set_len(num_actions * num_hands) };
        let result = fma_slices_uninit(result, &strategy, &cfv_actions);

        if game.compression_enabled() {
            let scale = node.strategy_scale();
            let decoder = params.gamma * scale / u16::MAX as f32;
            let cumulative_strategy = node.strategy_compressed_mut();

            strategy.iter_mut().zip(&*cumulative_strategy).for_each(|(x, y)| {
                *x += (*y as f32) * decoder;
            });

            if !locking.is_empty() {
                strategy.iter_mut().zip(locking).for_each(|(d, s)| {
                    if s.is_sign_positive() {
                        *d = 0.0;
                    }
                });
            }

            let new_scale = encode_unsigned_slice(cumulative_strategy, &strategy);
            node.set_strategy_scale(new_scale);

            let scale = node.regret_scale();
            let alpha_decoder = params.alpha * scale / i16::MAX as f32;
            let beta_decoder = params.beta * scale / i16::MAX as f32;
            let cumulative_regret = node.regrets_compressed_mut();

            cfv_actions.iter_mut().zip(&*cumulative_regret).for_each(|(x, y)| {
                *x += *y as f32 * if *y > 0 { alpha_decoder } else { beta_decoder };
            });

            cfv_actions.chunks_exact_mut(num_hands).for_each(|row| {
                sub_slice(row, result);
            });

            if !locking.is_empty() {
                cfv_actions.iter_mut().zip(locking).for_each(|(d, s)| {
                    if s.is_sign_positive() {
                        *d = 0.0;
                    }
                });
            }

            let new_scale = encode_signed_slice(cumulative_regret, &cfv_actions);
            node.set_regret_scale(new_scale);
        
        } else {

            let gamma = params.gamma;
            let cumulative_strategy = node.strategy_mut();
            cumulative_strategy.iter_mut().zip(&strategy).for_each(|(x, y)| {
                *x = *x * gamma + *y;
            });

            let (alpha, beta) = (params.alpha, params.beta);
            let cumulative_regret = node.regrets_mut();
            cumulative_regret.iter_mut().zip(&*cfv_actions).for_each(|(x, y)| {
                let coeff = if x.is_sign_positive() { alpha } else { beta };
                *x = *x * coeff + *y;
            });

            cumulative_regret.chunks_exact_mut(num_hands).for_each(|row| {
                sub_slice(row, result);
            });
        }
    
    } else {

        let mut cf_reach_actions = if game.compression_enabled() {
            regret_matching_compressed(node.regrets_compressed(), num_actions)
        } else {
            regret_matching(node.regrets(), num_actions)
        };

        let locking = game.locking_strategy(node);
        apply_locking_strategy(&mut cf_reach_actions, locking);

        let row_size = cf_reach.len();
        cf_reach_actions.chunks_exact_mut(row_size).for_each(|row| {
            mul_slice(row, cf_reach);
        });

        node.child_op(|action| {
            solve_cfv(
                game,
                &mut node.play(action),
                row_mut(cfv_actions.lock().spare_capacity_mut(), action, num_hands),
                player,
                row(&cf_reach_actions, action, row_size),
                params,
            );
        });

        let mut cfv_actions = cfv_actions.lock();
        unsafe { cfv_actions.set_len(num_actions * num_hands) };
        sum_slices_uninit(result, &cfv_actions);
    }
}

fn regret_matching(regret: &[f32], num_actions: usize) -> Vec<f32> {
    let mut strategy = Vec::with_capacity(regret.len());
    let uninit = strategy.spare_capacity_mut();
    uninit.iter_mut().zip(regret).for_each(|(s, &r)| {
        s.write(r.max(0.0));
    });
    unsafe { strategy.set_len(regret.len()) };

    let row_size = regret.len() / num_actions;
    let mut denom = Vec::with_capacity(row_size);
    sum_slices_uninit(denom.spare_capacity_mut(), &strategy);
    unsafe { denom.set_len(row_size) };

    let default = 1.0 / num_actions as f32;
    strategy.chunks_exact_mut(row_size).for_each(|row| {
        div_slice(row, &denom, default);        
    });

    strategy
}

fn regret_matching_compressed(regret: &[i16], num_actions: usize) -> Vec<f32> {
    let mut strategy = Vec::with_capacity(regret.len());
    strategy.extend(regret.iter().map(|&r| r.max(0) as f32));

    let row_size = strategy.len() / num_actions;
    let mut denom = Vec::with_capacity(row_size);
    sum_slices_uninit(denom.spare_capacity_mut(), &strategy);
    unsafe { denom.set_len(row_size) };

    let default = 1.0 / num_actions as f32;
    strategy.chunks_exact_mut(row_size).for_each(|row| {
        div_slice(row, &denom, default)
    });

    strategy
}

pub fn finalise<G: Game>(game: &mut G) {
    
    for player in 0..2 {
        let mut cfv = Vec::with_capacity(game.num_private_hands(player));
        compute_cfv(
            game,
            &mut game.root(),
            cfv.spare_capacity_mut(),
            player,
            game.intial_weights(player ^ 1),
            true,
        );
    }

    game.set_solved();
}
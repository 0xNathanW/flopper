use std::mem::MaybeUninit;
use crate::{game::Game, node::GameNode, latch::Latch, slice_ops::*};

// Counterfactural value of best response strategy.
pub fn compute_optimal_cfv<G: Game>(
    game: &G,
    node: &G::Node,
    result: &mut [MaybeUninit<f32>],
    player: usize,
    cf_reach: &[f32],
) {

    if node.is_terminal() {
        game.evaluate(node, player, result, cf_reach);
        return;
    }

    let num_actions = node.num_actions();
    let num_hands = game.num_private_hands(player);

    if num_actions == 1 && !node.is_chance() {
        let child = &node.play(0);
        compute_optimal_cfv(game, child, result, player, cf_reach);
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
            compute_optimal_cfv(
                game,
                &node.play(action),
                row_mut(cfv_actions.lock().spare_capacity_mut(), action, num_hands),
                player,
                &cf_reach_updated,
            )
        });

        let mut result_f64 = Vec::with_capacity(num_hands);
        let mut cfv_actions = cfv_actions.lock();
        unsafe { cfv_actions.set_len(num_actions * num_hands); };
        sum_slices_uninit_f64(result_f64.spare_capacity_mut(), &cfv_actions);
        unsafe { result_f64.set_len(num_hands); }

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
            compute_optimal_cfv(
                game,
                &node.play(action),
                row_mut(cfv_actions.lock().spare_capacity_mut(), action, num_hands),
                player,
                cf_reach,
            )
        });

        let locking = game.locking_strategy(node);
        let mut cfv_actions = cfv_actions.lock();
        unsafe { cfv_actions.set_len(num_actions * num_hands); }

        if locking.is_empty() {
            max_slices_uninit(result, &cfv_actions);
        } else {
            max_fma_slices_uninit(result, &cfv_actions, locking);
        }

    } else { // Opponent node

        let mut cf_reach_actions = if game.compression_enabled() {
            normalised_stategy_compressed(node.strategy_compressed(), num_actions)
        } else {
            normalised_strategy(node.strategy(), num_actions)
        };

        let locking = game.locking_strategy(node);
        apply_locking_strategy(&mut cf_reach_actions, locking);

        // Update reach probabilities.
        let row_size = cf_reach.len();
        cf_reach_actions.chunks_exact_mut(row_size).for_each(|row| {
            mul_slice(row, cf_reach);
        });

        // Calculate the counterfactual values for each action.
        node.child_op(|action| {
            compute_optimal_cfv(
                game, 
                &node.play(action),
                row_mut(cfv_actions.lock().spare_capacity_mut(), action, num_hands),
                player,
                row(&cf_reach_actions, action, row_size),
            );
        });

        // Sum counterfactual values.
        let mut cfv_actions = cfv_actions.lock();
        unsafe { cfv_actions.set_len(num_actions * num_hands) };
        sum_slices_uninit(result, &cfv_actions);
    }
}

pub fn compute_cfv<G: Game>(
    game: &G,
    node: &mut G::Node,
    result: &mut [MaybeUninit<f32>],
    player: usize,
    cf_reach: &[f32],
    save_cfvalues: bool,
) {

    if node.is_terminal() {
        game.evaluate(node, player, result, cf_reach);
        return;
    }

    let num_actions = node.num_actions();
    let num_hands = result.len();

    // Allocate memory for the counterfactual values.
    let cfv_actions = Latch::new(Vec::with_capacity(num_actions * num_hands));

    if node.is_chance() {
        // Update reach probabilities.
        let mut cf_reach_updated = Vec::with_capacity(cf_reach.len());
        mul_slice_scalar_uninit(
            cf_reach_updated.spare_capacity_mut(), 
            cf_reach, 
            1.0 / game.chance_factor(node) as f32
        );
        unsafe { cf_reach_updated.set_len(cf_reach.len()) };

        // Calculate the counterfactual values for each action.
        node.child_op(|action| {
            compute_cfv(
                game,
                &mut node.play(action),
                row_mut(cfv_actions.lock().spare_capacity_mut(), action, num_hands),
                player,
                &cf_reach_updated,
                save_cfvalues,
            );
        });

        let mut result_f64 = Vec::with_capacity(num_hands);

        // Sum counterfactual values.
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

        // Save the counterfactual values.
        if save_cfvalues && node.cf_values_storage_player() == Some(player) {
            let result = unsafe { &*(result as *const _ as *const [f32]) };
            
            if game.compression_enabled() {
                let cfv_scale = encode_signed_slice(node.cf_values_chance_compressed_mut(), result);
                node.set_cf_value_chance_scale(cfv_scale);
            } else {
                node.cf_values_chance_mut().copy_from_slice(result);
            }
        }
    
    } else if node.player() == player {

        node.child_op(|action| {
            compute_cfv(
                game,
                &mut node.play(action),
                row_mut(cfv_actions.lock().spare_capacity_mut(), action, num_hands),
                player,
                cf_reach,
                save_cfvalues,
            );
        });

        let mut strategy = if game.compression_enabled() {
            normalised_stategy_compressed(node.strategy_compressed(), num_actions)
        } else {
            normalised_strategy(node.strategy(), num_actions)
        };

        let locking = game.locking_strategy(node);
        apply_locking_strategy(&mut strategy, locking);

        // Sum counterfactual values.
        let mut cfv_actions = cfv_actions.lock();
        unsafe { cfv_actions.set_len(num_actions * num_hands) };
        fma_slices_uninit(result, &strategy, &cfv_actions);
        
        if save_cfvalues {
            if game.compression_enabled() {
                let cfv_scale = encode_signed_slice(node.cf_values_compressed_mut(), &cfv_actions);
                node.set_cf_value_scale(cfv_scale);
            } else {
                node.cf_values_mut().copy_from_slice(&cfv_actions);
            }
        }
    
    } else if num_actions == 1 {
        compute_cfv(
            game,
            &mut node.play(0),
            result,
            player,
            cf_reach,
            save_cfvalues,
        );
    
    } else {

        let mut cf_reach_actions = if game.compression_enabled() {
            normalised_stategy_compressed(node.strategy_compressed(), num_actions)
        } else {
            normalised_strategy(node.strategy(), num_actions)
        };

        let locking = game.locking_strategy(node);
        apply_locking_strategy(&mut cf_reach_actions, locking);

        // Update reach probabilities.
        let row_size = cf_reach.len();
        cf_reach_actions.chunks_exact_mut(row_size).for_each(|row| {
            mul_slice(row, cf_reach);
        });

        // Calculate the counterfactual values for each action.
        node.child_op(|action| {
            compute_cfv(
                game,
                &mut node.play(action),
                row_mut(cfv_actions.lock().spare_capacity_mut(), action, num_hands),
                player,
                row(&cf_reach_actions, action, row_size),
                save_cfvalues,
            );
        });

        // Sum counterfactual values.
        let mut cfv_actions = cfv_actions.lock();
        unsafe { cfv_actions.set_len(num_actions * num_hands) };
        sum_slices_uninit(result, &cfv_actions);
    }

    // Save counterfactual values for IP.
    if save_cfvalues && node.has_ip_cf_values() && player == 1 {
        let result = unsafe { &*(result as *const _ as *const [f32]) };
        if game.compression_enabled() {
            let cfv_scale = encode_signed_slice(node.ip_cf_values_compressed_mut(), result);
            node.set_ip_cf_value_scale(cfv_scale);
        } else {
            node.ip_cf_values_mut().copy_from_slice(result);
        }
    }
}

pub fn normalised_stategy_compressed(strategy: &[u16], num_actions: usize) -> Vec<f32> {

    let mut normalised = Vec::with_capacity(strategy.len());
    let uninit = normalised.spare_capacity_mut();

    uninit.iter_mut().zip(strategy).for_each(|(n, s)| {
        n.write(*s as f32);
    });
    unsafe { normalised.set_len(strategy.len()) };

    let row_size = strategy.len() / num_actions;
    let mut denom = Vec::with_capacity(row_size);
    sum_slices_uninit(denom.spare_capacity_mut(), &normalised);
    unsafe { denom.set_len(row_size) };

    let default = 1.0 / num_actions as f32;
    normalised.chunks_exact_mut(row_size).for_each(|row| {
        div_slice(row, &denom, default);
    });

    normalised
}

pub fn normalised_strategy(strategy: &[f32], num_actions: usize) -> Vec<f32> {

    let mut normalised = Vec::with_capacity(strategy.len());
    let uninit = normalised.spare_capacity_mut();

    let row_size = strategy.len() / num_actions;
    let mut denom = Vec::with_capacity(row_size);
    sum_slices_uninit(denom.spare_capacity_mut(), strategy);
    unsafe { denom.set_len(row_size) };

    let default = 1.0 / num_actions as f32;
    uninit.chunks_exact_mut(row_size)
        .zip(strategy.chunks_exact(row_size))
        .for_each(|(n, s)| {
            div_slice_uninit(n, s, &denom, default);
        });

    unsafe { normalised.set_len(strategy.len()) };
    normalised
}


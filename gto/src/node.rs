use std::ops::Range;
use rayon::prelude::*;
use crate::latch::LatchGuard;


pub trait GameNode: Send + Sync {
    
    // Perform an operation on each child node.
    fn child_op<OP: Fn(usize) + Sync + Send>(&self, op: OP) {
        self.action_idxs().into_iter().for_each(op)
    }

    fn is_terminal(&self) -> bool;

    fn is_chance(&self) -> bool;

    fn player(&self) -> usize;

    fn num_actions(&self) -> usize;

    // Return the next node after playing the action.
    fn play(&self, action: usize) -> LatchGuard<Self>;

    fn strategy(&self) -> &[f32];

    fn strategy_mut(&mut self) -> &mut [f32];

    // Cumulative regrets.
    fn regrets(&self) -> &[f32];

    fn regrets_mut(&mut self) -> &mut [f32];

    fn cf_values(&self) -> &[f32];

    fn cf_values_mut(&mut self) -> &mut [f32];

    fn has_ip_cf_values(&self) -> bool {
        false
    }

    fn ip_cf_values(&self) -> &[f32] {
        unreachable!()
    }

    fn ip_cf_values_mut(&mut self) -> &mut [f32] {
        unreachable!()
    }

    fn cf_values_storage_player(&self) -> Option<usize> {
        None
    }

    fn cf_values_chance(&self) -> &[f32] {
        unreachable!()
    }

    fn cf_values_chance_mut(&mut self) -> &mut [f32] {
        unreachable!()
    }

    fn action_idxs(&self) -> Range<usize> {
        0..self.num_actions()
    }

    fn strategy_compressed(&self) -> &[u16] {
        unreachable!()
    }

    fn strategy_compressed_mut(&mut self) -> &mut [u16] {
        unreachable!()
    }

    fn regrets_compressed(&self) -> &[i16] {
        unreachable!()
    }

    fn regrets_compressed_mut(&mut self) -> &mut [i16] {
        unreachable!()
    }

    fn cf_values_compressed(&self) -> &[i16] {
        unreachable!()
    }

    fn cf_values_compressed_mut(&mut self) -> &mut [i16] {
        unreachable!()
    }

    fn ip_cf_values_compressed(&self) -> &[i16] {
        unreachable!()
    }

    fn ip_cf_values_compressed_mut(&mut self) -> &mut [i16] {
        unreachable!()
    }

    fn cf_values_chance_compressed(&self) -> &[i16] {
        unreachable!()
    }

    fn cf_values_chance_compressed_mut(&mut self) -> &mut [i16] {
        unreachable!()
    }

    fn strategy_scale(&self) -> f32 {
        unreachable!()
    }

    fn set_strategy_scale(&mut self, _scale: f32) {
        unreachable!()
    }

    fn regret_scale(&self) -> f32 {
        unreachable!()
    }

    fn set_regret_scale(&mut self, _scale: f32) {
        unreachable!()
    }

    fn cf_value_scale(&self) -> f32 {
        unreachable!()
    }

    fn set_cf_value_scale(&mut self, _scale: f32) {
        unreachable!()
    }

    fn ip_cf_value_scale(&self) -> f32 {
        unreachable!()
    }

    fn set_ip_cf_value_scale(&mut self, _scale: f32) {
        unreachable!()
    }

    fn cf_value_chance_scale(&self) -> f32 {
        unreachable!()
    }

    fn set_cf_value_chance_scale(&mut self, _scale: f32) {
        unreachable!()
    }

    fn enable_parallelisation(&self) -> bool {
        false
    }
}
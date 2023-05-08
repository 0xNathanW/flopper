use core::slice;
use crate::{node::GameNode, player::*, latch::{Latch, LatchGuard}};
use super::PostFlopNode;

impl GameNode for PostFlopNode {

    fn is_terminal(&self) -> bool {
        self.player & PLAYER_TERMINAL_FLAG != 0
    }   

    fn is_chance(&self) -> bool {
        self.player & PLAYER_CHANCE_FLAG != 0
    }

    fn player(&self) -> usize {
        self.player as usize
    }

    fn num_actions(&self) -> usize {
        self.num_children as usize
    }

    fn play(&self, action: usize) -> LatchGuard<Self> {
        self.children()[action].lock()
    }

    fn cf_values_storage_player(&self) -> Option<usize> {
        match self.player & PLAYER_MASK {
            0 => Some(1),
            1 => Some(0),
            _ => None,
        }
    }

    fn strategy(&self) -> &[f32] {
        unsafe {
            slice::from_raw_parts(
                self.storage_1 as *const f32, 
                self.num_elements as usize
            )
        }
    }

    fn strategy_mut(&mut self) -> &mut [f32] {
        unsafe {
            slice::from_raw_parts_mut(
                self.storage_1 as *mut f32, 
                self.num_elements as usize
            )
        }
    }

    fn regrets(&self) -> &[f32] {
        unsafe {
            slice::from_raw_parts(
                self.storage_2 as *const f32, 
                self.num_elements as usize
            )
        }
    }

    fn regrets_mut(&mut self) -> &mut [f32] {
        unsafe {
            slice::from_raw_parts_mut(
                self.storage_2 as *mut f32, 
                self.num_elements as usize
            )
        }
    }

    fn cf_values(&self) -> &[f32] {
        unsafe {
            slice::from_raw_parts(
                self.storage_2 as *const f32, 
                self.num_elements as usize
            )
        }
    }

    fn cf_values_mut(&mut self) -> &mut [f32] {
        unsafe {
            slice::from_raw_parts_mut(
                self.storage_2 as *mut f32, 
                self.num_elements as usize
            )
        }
    }

    fn has_ip_cf_values(&self) -> bool {
        self.num_elements_ip != 0
    }

    fn ip_cf_values(&self) -> &[f32] {
        unsafe {
            slice::from_raw_parts(
                self.storage_3 as *const f32, 
                self.num_elements_ip as usize
            )
        }
    }

    fn ip_cf_values_mut(&mut self) -> &mut [f32] {
        unsafe {
            slice::from_raw_parts_mut(
                self.storage_3 as *mut f32, 
                self.num_elements_ip as usize
            )
        }
    }

    fn cf_values_chance(&self) -> &[f32] {
        unsafe {
            slice::from_raw_parts(
                self.storage_1 as *const f32, 
                self.num_elements as usize
            )
        }
    }

    fn cf_values_chance_mut(&mut self) -> &mut [f32] {
        unsafe {
            slice::from_raw_parts_mut(
                self.storage_1 as *mut f32, 
                self.num_elements as usize
            )
        }
    }

    fn strategy_compressed(&self) -> &[u16] {
        unsafe {
            slice::from_raw_parts(
                self.storage_1 as *const u16, 
                self.num_elements as usize
            )
        }
    }

    fn strategy_compressed_mut(&mut self) -> &mut [u16] {
        unsafe {
            slice::from_raw_parts_mut(
                self.storage_1 as *mut u16, 
                self.num_elements as usize
            )
        }
    }

    fn regrets_compressed(&self) -> &[i16] {
        unsafe {
            slice::from_raw_parts(
                self.storage_2 as *const i16, 
                self.num_elements as usize
            )
        }
    }

    fn regrets_compressed_mut(&mut self) -> &mut [i16] {
        unsafe {
            slice::from_raw_parts_mut(
                self.storage_2 as *mut i16, 
                self.num_elements as usize
            )
        }
    }

    fn cf_values_compressed(&self) -> &[i16] {
        unsafe {
            slice::from_raw_parts(
                self.storage_2 as *const i16, 
                self.num_elements as usize
            )
        }
    }

    fn cf_values_compressed_mut(&mut self) -> &mut [i16] {
        unsafe {
            slice::from_raw_parts_mut(
                self.storage_2 as *mut i16, 
                self.num_elements as usize
            )
        }
    }

    fn ip_cf_values_compressed(&self) -> &[i16] {
        unsafe {
            slice::from_raw_parts(
                self.storage_3 as *const i16, 
                self.num_elements_ip as usize
            )
        }
    }

    fn ip_cf_values_compressed_mut(&mut self) -> &mut [i16] {
        unsafe {
            slice::from_raw_parts_mut(
                self.storage_3 as *mut i16, 
                self.num_elements_ip as usize
            )
        }
    }

    fn cf_values_chance_compressed(&self) -> &[i16] {
        unsafe {
            slice::from_raw_parts(
                self.storage_1 as *const i16, 
                self.num_elements as usize
            )
        }
    }

    fn cf_values_chance_compressed_mut(&mut self) -> &mut [i16] {
        unsafe {
            slice::from_raw_parts_mut(
                self.storage_1 as *mut i16, 
                self.num_elements as usize
            )
        }
    }

    fn strategy_scale(&self) -> f32 {
        self.scale_1
    }

    fn set_strategy_scale(&mut self, scale: f32) {
        self.scale_1 = scale;
    }

    fn regret_scale(&self) -> f32 {
        self.scale_2
    }

    fn set_regret_scale(&mut self, scale: f32) {
        self.scale_2 = scale;
    }

    fn cf_value_scale(&self) -> f32 {
        self.scale_2
    }

    fn set_cf_value_scale(&mut self, scale: f32) {
        self.scale_2 = scale;
    }

    fn ip_cf_value_scale(&self) -> f32 {
        self.scale_3
    }

    fn set_ip_cf_value_scale(&mut self, scale: f32) {
        self.scale_3 = scale;
    }

    fn cf_value_chance_scale(&self) -> f32 {
        self.scale_1
    }

    fn set_cf_value_chance_scale(&mut self, scale: f32) {
        self.scale_1 = scale;
    }

    fn enable_parallelisation(&self) -> bool {
        !self.river.is_dealt()
    }
}

impl PostFlopNode {

    pub fn children(&self) -> &[Latch<Self>] {
        let ptr = self as *const _ as *const Latch<Self>;
        unsafe {
            slice::from_raw_parts(
                ptr.add(self.children_offset as usize), 
                self.num_children as usize
            )
        }        
    }

}
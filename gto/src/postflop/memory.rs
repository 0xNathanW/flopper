use std::mem;
use crate::{Street, node::GameNode};

use super::{PostFlopGame, ProcessState};

impl PostFlopGame {

    // Returns the number of bytes used by the game tree.
    pub fn internal_mem_usage(&self) -> u64 {

        let mut size = mem::size_of::<Self>() as u64;
        
        // Add vector sizes.
        size += vec_mem_size(&self.added_lines);
        for v in &self.added_lines {
            size += vec_mem_size(v);
        }
        size += vec_mem_size(&self.removed_lines);
        for v in &self.removed_lines {
            size += vec_mem_size(v);
        }

        size += vec_mem_size(&self.valid_idxs_turn);
        size += vec_mem_size(&self.valid_idxs_river);

        for v in &self.isomorphism_ref_river {
            size += vec_mem_size(v);
        }

        for v in &self.isomorphism_card_river {
            size += vec_mem_size(v);
        }

        size += vec_mem_size(&self.node_arena);

        for p in 0..2 {
            size += vec_mem_size(&self.hands[p]);
            size += vec_mem_size(&self.initial_weights[p]);
            size += vec_mem_size(&self.same_hand_idx[p]);
            size += vec_mem_size(&self.valid_idxs_flop[p]);

            for v in &self.valid_idxs_turn {
                size += vec_mem_size(&v[p]);
            }

            for v in &self.valid_idxs_river {
                size += vec_mem_size(&v[p]);
            }

            for v in &self.hand_strengths {
                size += vec_mem_size(&v[p]);
            }

            for v in &self.isomorphism_swap_turn {
                size += vec_mem_size(&v[p]);
            }

            for v in &self.isomorphism_swap_river {
                for u in v {
                    size += vec_mem_size(&u[p]);
                }
            }
        }

        size
    }

    pub fn allocate_memory(&mut self, compression: bool) {
        // Cannot allocate memory if game tree is not initialised.
        if self.state <= ProcessState::Uninitialised {
            panic!("Game tree not initialised.");
        }

        // Memory already allocated.
        if self.state == ProcessState::MemoryAllocated
        && self.storage_mode == Street::River
        && self.compression_enabled == compression {
            return;
        }

        let n_bytes = if compression { 2 } else { 4 };
        // Check if memory usage is too high.
        if n_bytes * self.num_storage > isize::MAX as u64 
        || n_bytes * self.num_storage_chance > isize::MAX as u64 {
            panic!("Memory usage too high.");
        }

        self.state = ProcessState::MemoryAllocated;
        self.compression_enabled = compression;
        self.clear_storage();

        let storage_bytes = (n_bytes * self.num_storage) as usize;
        let storage_ip_bytes = (n_bytes * self.num_storage_ip) as usize;
        let storage_chance_bytes = (n_bytes * self.num_storage_chance) as usize;
        
        // Allocate storage memory.
        self.storage_1 = vec![0; storage_bytes];
        self.storage_2 = vec![0; storage_bytes];
        self.storage_ip = vec![0; storage_ip_bytes];
        self.storage_chance = vec![0; storage_chance_bytes];

        self.allocate_memory_nodes();

        self.storage_mode = Street::River;
        self.target_storage_mode = Street::River;
    }

    fn allocate_memory_nodes(&mut self) {
        
        let n_bytes = if self.compression_enabled { 2 } else { 4 };
        let mut action_counter = 0;
        let mut ip_counter = 0;
        let mut chance_counter = 0;

        for node in self.node_arena.iter() {
            let mut node = node.lock();

            if node.is_terminal() {
                // Base case.

            } else if node.is_chance() {
                unsafe {
                    node.storage_1 = self.storage_chance.as_mut_ptr().add(chance_counter);
                }
                chance_counter += n_bytes * node.num_elements as usize;
            
            } else {
                unsafe {
                    node.storage_1 = self.storage_1.as_mut_ptr().add(action_counter);
                    node.storage_2 = self.storage_2.as_mut_ptr().add(action_counter);
                    node.storage_3 = self.storage_ip.as_mut_ptr().add(ip_counter);
                }
                action_counter += n_bytes * node.num_elements as usize;
                ip_counter += n_bytes * node.num_elements_ip as usize;
            }
        }
    }

    pub fn clear_storage(&mut self) {
        self.storage_1.clear();
        self.storage_2.clear();
        self.storage_ip.clear();
        self.storage_chance.clear();
    }
}

fn vec_mem_size<T>(v: &Vec<T>) -> u64 {
    let unit_size = mem::size_of::<T>() as u64;
    unit_size * v.capacity() as u64
}
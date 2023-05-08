use std::collections::BTreeMap;
use poker::{Range, Card, Hand, Board};
use crate::{action::*, player::*, Street, latch::Latch};

mod game;
mod node;
mod init;
mod memory;
mod evaluate;
mod interpreter;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub enum ProcessState {
    ConfigError,
    #[default]
    Uninitialised,
    TreeBuilt,
    MemoryAllocated,
    Solved,
}

#[derive(Debug, Default)]
pub struct PostFlopGame {
    // Current proccess state of game.
    state: ProcessState,

    ranges: [Range; 2],
    board:  Board,

    tree_config:    TreeConfig,
    added_lines:    Vec<Vec<Action>>,
    removed_lines:  Vec<Vec<Action>>,
    // Root node of action tree.
    action_root:    Box<Latch<ActionTreeNode>>,

    num_combos:         f64,
    // Initial weights given by range.
    initial_weights:    [Vec<f32>; 2],
    // Hands for each player.
    hands:              [Vec<Hand>; 2],
    // Index of player1'hand in player2's hand array and vice versa.
    same_hand_idx:      [Vec<u16>; 2],
    
    // Contains all valid hand indexes for each street.
    valid_idxs_flop:    [Vec<u16>; 2],
    // First vec indexed by the turn card.
    valid_idxs_turn:    Vec<[Vec<u16>; 2]>,
    // First vec indexed by Hand(turn, river).range_idx().
    valid_idxs_river:   Vec<[Vec<u16>; 2]>,
    
    hand_strengths:         Vec<[Vec<(u16, u16)>; 2]>,

    // Isomorphism in suits mean suits can be interchanged
    // without affecting strategic structure of the game.

    // Holds cards that are isomorphic on turn/river.
    isomorphism_card_turn:  Vec<u8>,
    isomorphism_card_river: [Vec<u8>; 4],
    // Reference index of card isomorphism on turn/river cards.
    isomorphism_ref_turn:   Vec<u8>,
    isomorphism_ref_river:  Vec<Vec<u8>>,
    // Maps for suits on turn/river for each player
    // (idx in hands, idx in range) for hands with isomorphic suits.
    isomorphism_swap_turn:  [[Vec<(u16, u16)>; 2]; 4],
    isomorphism_swap_river: [[[Vec<(u16, u16)>; 2]; 4]; 4],

    bunching_num_dead:      usize,
    bunching_num_combos:    f64,
    bunching_arena:         Vec<f32>,
    bunching_strength:      Vec<[Vec<u16>; 2]>,
    bunching_num_flop:      [Vec<usize>; 2],
    bunching_num_turn:      [Vec<Vec<usize>>; 2],
    bunching_num_river:     [Vec<Vec<usize>>; 2],
    bunching_coeff_flop:    [Vec<usize>; 2],
    bunching_coeff_turn:    [Vec<Vec<usize>>; 2],

    storage_mode:           Street,
    target_storage_mode:    Street,
    num_nodes:              [u64; 3],
    compression_enabled:    bool,
    num_storage:            u64,
    num_storage_ip:         u64,
    num_storage_chance:     u64,
    misc_memory_usage:      u64,

    // Arena to access game tree nodes easily.
    node_arena:             Vec<Latch<PostFlopNode>>,
    storage_1:              Vec<u8>,
    storage_2:              Vec<u8>,
    storage_ip:             Vec<u8>,
    storage_chance:         Vec<u8>,
    locking_strat:          BTreeMap<usize, Vec<f32>>,

    action_history:         Vec<usize>,
    node_history:           Vec<usize>,
    normalised_cache:       bool,
    turn:                   Card,
    river:                  Card,
    turn_swapped_suit:      Option<(u8, u8)>,
    turn_swap:              Option<u8>,
    river_swap:             Option<(u8, u8)>,
    total_bet_amount:       [u32; 2],
    weights:                [Vec<f32>; 2],
    normalised_weights:     [Vec<f32>; 2],
    cf_values_cache:        [Vec<f32>; 2],
}

unsafe impl Send for PostFlopGame {}
unsafe impl Sync for PostFlopGame {}

#[derive(Debug)]
pub struct PostFlopNode {

    last_action:  Action,
    player:       u8,

    turn:   Card,
    river:  Card,

    locked: bool,

    amount: u32,

    children_offset: u32,

    num_children:       u16,
    num_elements_ip:    u16,
    num_elements:       u32,

    scale_1: f32,
    scale_2: f32,
    scale_3: f32,

    storage_1: *mut u8, // Strategy.
    storage_2: *mut u8, // Regrets/Counterfactual values.
    storage_3: *mut u8, // IP counterfactual values.
}

impl Default for PostFlopNode {
    fn default() -> PostFlopNode {
        PostFlopNode {
            last_action: Action::None,
            player: PLAYER_OOP,
            turn: Card::default(),
            river: Card::default(),
            locked: false,
            amount: 0,
            children_offset: 0,
            num_children: 0,
            num_elements_ip: 0,
            num_elements: 0,
            scale_1: 0.0,
            scale_2: 0.0,
            scale_3: 0.0,
            storage_1: std::ptr::null_mut(),
            storage_2: std::ptr::null_mut(),
            storage_3: std::ptr::null_mut(),
        }
    }
}

unsafe impl Send for PostFlopNode {}
unsafe impl Sync for PostFlopNode {}

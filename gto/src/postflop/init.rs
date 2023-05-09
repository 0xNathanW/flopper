use poker::{
    range::Range, Board, Card, Hand, card::Suit, 
    evaluate::{rank_hand_7, load_lookup_table},
};
use crate::{
    ConfigError, 
    action::{ActionTree, ActionTreeNode, Action}, 
    Street,
    node::GameNode,
    game::Game, 
    player::*, 
    latch::Latch, 
};
use super::{PostFlopGame, ProcessState, PostFlopNode};

#[derive(Debug, Default)]
struct BuildTreeData {
    flop_idx:           usize,
    turn_idx:           usize,
    river_idx:          usize,
    num_storage:        u64,
    num_storage_ip:     u64,
    num_storage_chance: u64
}

impl PostFlopGame {

    pub fn new(ranges: [Range; 2], board: Board, action_tree: ActionTree) -> Result<PostFlopGame, ConfigError> {

        if !action_tree.invalid_terminals().is_empty() {
            return Err(ConfigError::InvalidTerminalNode);
        }

        if !board.is_flop_dealt() {
            return Err(ConfigError::MissingFlop);
        }
        
        let board_street = {
            if board.is_river_dealt() {
                Street::River
            } else if board.is_turn_dealt() {
                Street::Turn
            } else {
                Street::Flop
            }
        };

        // Make sure board matches config street.
        if board_street != action_tree.config.initial_street {
            return Err(ConfigError::MismatchedStreets);
        }

        let mut game = PostFlopGame::default();
        // Initiate fields from config.
        game.state          = ProcessState::ConfigError; // Will change as game tree is buil
        game.board          = board;
        game.ranges         = ranges;
        game.tree_config    = action_tree.config;
        game.added_lines    = action_tree.added_lines;
        game.removed_lines  = action_tree.removed_lines;
        game.action_root    = action_tree.root;
        
        // Set initial weights and private cards.
        for player in 0..2 {
            // Get all possible hands excluding board cards.
            let combos = game.ranges[player].hand_combos_dead(game.board.mask());
            let (hands, weights) = combos.into_iter().unzip();
            game.initial_weights[player] = weights;
            game.hands[player] = hands;
        }

        game.num_combos = 0.0;
        for (&oop_hand, &weight) in game.hands[0].iter().zip(game.initial_weights[0].iter()) {
            
            for (&ip_hand, &ip_weight) in game.hands[1].iter().zip(game.initial_weights[1].iter()) {
                // Add combo if no overlap in hands.
                if oop_hand.mask() & ip_hand.mask() == 0 {
                    game.num_combos += weight as f64 * ip_weight as f64;
                }
            }
        }

        if game.num_combos == 0.0 {
            panic!("No valid combos found.");
        }

        for player in 0..2 {
            
            let same_hand_idx = &mut game.same_hand_idx[player];
            let player_hands = &game.hands[player];
            let opp_hands = &game.hands[player ^ 1];

            // Cycle through players hands, if hand exists in opponent's hands, add index to same_hand_idx.
            for hand in player_hands {
                same_hand_idx.push(
                    opp_hands
                        .binary_search(hand)
                        .map_or(u16::MAX, |idx| idx as u16)
                );
            }
        }

        game.init_valid_idxs();
        game.init_hand_strengths();
        game.init_isomorphism();
        game.init_root()?;
        game.state = ProcessState::TreeBuilt;
        
        let v = [
            vec![0.0; game.num_private_hands(0)],
            vec![0.0; game.num_private_hands(1)],
        ];
        game.weights = v.clone(); 
        game.normalised_weights = v.clone();
        game.cf_values_cache = v.clone();
        
        game.reset_bunching();
        Ok(game)
    }

    // Returns which hand indexes are valid (don't conflict with board) for each possible future board.
    fn init_valid_idxs(&mut self) {

        let flop_idxs = if !self.board.is_turn_dealt() {
            valid_idxs_street(&self.hands, Card::default(), Card::default())
        } else {
            [Vec::new(), Vec::new()]
        };
    
        let mut turn_idxs = vec![[Vec::new(), Vec::new()]; 52];
        for t in 0..52 {
            let t = Card(t);
            
            if !self.board.flop.contains(&t) 
                && (!self.board.is_turn_dealt() || t == self.board.turn) 
                && !self.board.is_river_dealt() 
            {
                turn_idxs[t.0 as usize] = valid_idxs_street(&self.hands, t, Card::default()); 
            }
        }
    
        let mut river_idxs = vec![[Vec::new(), Vec::new()]; 1326];
        for t in 0..52 {
            for r in 0..52 {
                let t = Card(t);
                let r = Card(r);
    
                if !self.board.flop.contains(&t)
                    && !self.board.flop.contains(&r)
                    && (!self.board.is_turn_dealt() || t == self.board.turn || r == self.board.turn)
                    && (!self.board.is_river_dealt() || t == self.board.river || r == self.board.river)
                {
                    let idx = Hand(t, r).idx();
                    river_idxs[idx] = valid_idxs_street(&self.hands, t, r)
                }
            }
        }
    
        self.valid_idxs_flop = flop_idxs;
        self.valid_idxs_turn = turn_idxs;
        self.valid_idxs_river = river_idxs;
    }

    fn init_hand_strengths(&mut self) {
        
        let lookup_table = load_lookup_table().unwrap();
        let flop_mask = self.board.flop_mask();
        let mut hand_strengths = vec![Default::default(); 1326];
        let cards = [
            self.board.flop[2],
            self.board.flop[1],
            self.board.flop[0],
            Card::default(),
            Card::default(),
            Card::default(),
            Card::default(),
        ];

        for t in 0..52 {
            for r in (t + 1)..52 {

                if flop_mask & (1 << t) == 0
                && flop_mask & (1 << r) == 0
                && (!self.board.is_turn_dealt()  || self.board.turn.0 == t  || self.board.turn.0 == r)
                && (!self.board.is_river_dealt() || self.board.river.0 == t || self.board.river.0 == r)
                {

                    let mut cards = cards.clone();
                    cards[3] = Card(t);
                    cards[4] = Card(r);

                    let mut strength = [
                        Vec::with_capacity(self.hands[0].len() + 2),
                        Vec::with_capacity(self.hands[1].len() + 2),
                    ];


                    for player in 0..2 {

                        strength[player].push((0, 0));
                        strength[player].push((u16::MAX, u16::MAX));

                        for (idx, &hand) in self.hands[player].iter().enumerate() {
                            if cards.contains(&hand.0) || cards.contains(&hand.1) {
                                continue;
                            } else {
                                cards[5] = hand.0;
                                cards[6] = hand.1;
                                let rank = rank_hand_7(&cards, &lookup_table);
                                strength[player].push((rank + 1, idx as u16));
                                // dbg!(strength[player].len());
                                cards[5] = Card::default();
                                cards[6] = Card::default();
                            }
                        } 

                        strength[player].shrink_to_fit();
                        strength[player].sort_unstable();
                    }

                    hand_strengths[Hand(Card(t), Card(r)).idx()] = strength;
                }
            }
        }

        self.hand_strengths = hand_strengths;
    }

    fn init_isomorphism(&mut self) {

        let mut suit_iso = [0; 4];
        let mut next_idx = 1;

        'outer: for suit_2 in 1..4 {
            for suit_1 in 0..suit_2 {

                // If suit is isomorphic in both ranges, add to suit_iso.
                if self.ranges[0].suit_isomorphistic([suit_1.into(), suit_2.into()]) 
                && self.ranges[1].suit_isomorphistic([suit_1.into(), suit_2.into()]) 
                {
                    suit_iso[suit_2 as usize] = suit_iso[suit_1 as usize];
                    continue 'outer;
                }
            }
            
            // If no isomorphism for suit_2, set in suit_iso as self.
            suit_iso[suit_2 as usize] = next_idx;
            next_idx += 1;
        }

        let flop_mask = self.board.flop_mask();
        let mut flop_rankset = [0; 4];
        // Set ranks in each suit.
        for card in self.board.flop.iter() {
            flop_rankset[card.suit_u8() as usize] |= 1 << card.rank_u8();
        }
        
        let mut isomorphic_suit = [None; 4];
        // Maps hand range index to index in hands array.
        let mut reverse_table = vec![usize::MAX, 1326];

        let mut iso_ref_turn = Vec::new();
        let mut iso_card_turn = Vec::new();
        let mut iso_swap_turn: [[Vec<(u16, u16)>; 2]; 4] = Default::default();
        
        if !self.board.is_turn_dealt() {
            for suit_1 in 1..4 {
                for suit_2 in 0..suit_1 {
                    // If suits are isomorphic in context of flop and ranges, perform swap.
                    if flop_rankset[suit_1 as usize] == flop_rankset[suit_2 as usize] 
                    && suit_iso[suit_1 as usize] == suit_iso[suit_2 as usize]
                    {
                        isomorphic_suit[suit_1 as usize] = Some(suit_2);
                        iso_swap_internal(
                            &mut iso_swap_turn,
                            &mut reverse_table,
                            [suit_1.into(), suit_2.into()],
                            &self.hands,
                        );
                        break;
                    }
                }
            }

            iso_internal(
                &mut iso_ref_turn,
                &mut iso_card_turn,
                &isomorphic_suit,
                flop_mask,
            );
        }

        let mut iso_ref_river = vec![Vec::new(); 52];
        let mut iso_card_river: [Vec<u8>; 4] = Default::default();
        let mut iso_swap_river: [[[Vec<(u16, u16)>; 2]; 4]; 4] = Default::default();

        if !self.board.is_river_dealt() {
            for t in 0..52 {
                // Make sure river card not dealt.
                if flop_mask & (1 << t) != 0 || (self.board.is_turn_dealt() && self.board.turn.0 != t) {
                    continue;
                }

                let t_mask = flop_mask | (1 << t);
                let mut turn_rankset = flop_rankset;
                turn_rankset[t as usize & 3] |= 1 << (t >> 2);

                isomorphic_suit.fill(None);

                for suit_1 in 1..4 {
                    for suit_2 in 0..suit_1 {

                        if (flop_rankset[suit_1 as usize] == flop_rankset[suit_2 as usize] || self.board.is_turn_dealt())
                        && turn_rankset[suit_1 as usize] == turn_rankset[suit_2 as usize]
                        && suit_iso[suit_1 as usize] == suit_iso[suit_2 as usize]
                        {
                            isomorphic_suit[suit_1 as usize] = Some(suit_2);
                            iso_swap_internal(
                                &mut iso_swap_river[t as usize & 3],
                                &mut reverse_table,
                                [suit_1.into(), suit_2.into()],
                                &self.hands,
                            );
                            break;
                        }
                    }
                }

                iso_internal(
                    &mut iso_ref_river[t as usize],
                    &mut iso_card_river[t as usize & 3],
                    &isomorphic_suit,
                    t_mask,
                );
            }
        }

        self.isomorphism_ref_turn   = iso_ref_turn;
        self.isomorphism_card_turn  = iso_card_turn;
        self.isomorphism_swap_turn  = iso_swap_turn;
        self.isomorphism_ref_river  = iso_ref_river;
        self.isomorphism_card_river = iso_card_river;
        self.isomorphism_swap_river = iso_swap_river;
    }

    fn init_root(&mut self) -> Result<(), ConfigError> {

        let num_nodes = self.count_nodes();

        let total = num_nodes.iter().sum::<u64>();
        // Cap number of nodes at u32::MAX.
        if total > u32::MAX as u64 || std::mem::size_of::<PostFlopNode>() as u64 * total > isize::MAX as u64{
            return Err(ConfigError::TooManyNodes);
        }

        self.num_nodes = num_nodes;
        self.node_arena = (0..total).map(|_| Latch::new(PostFlopNode::default())).collect();
        self.clear_storage();

        let mut data = BuildTreeData {
            turn_idx: num_nodes[0] as usize,
            river_idx: (num_nodes[0] + num_nodes[1]) as usize,
            ..Default::default()
        };

        match self.tree_config.initial_street {
            Street::Flop  => data.flop_idx  += 1,
            Street::Turn  => data.turn_idx  += 1,
            Street::River => data.river_idx += 1,
        }

        let mut root = self.node_arena[0].lock();
        root.turn = self.board.turn;
        root.river = self.board.river;

        self.build_tree(0, &self.action_root.lock(), &mut data);

        self.num_storage = data.num_storage;
        self.num_storage_ip = data.num_storage_ip;
        self.num_storage_chance = data.num_storage_chance;
        self.misc_memory_usage = self.internal_mem_usage();

        Ok(())        
    }

    fn build_tree(&self, node_idx: usize, action_node: &ActionTreeNode, data: &mut BuildTreeData) {
        
        let mut node = self.node_arena[node_idx].lock();
        node.player = action_node.player;
        node.amount = action_node.amount;

        if action_node.is_terminal() {
            // Base case.
            return;
        }
        
        if action_node.is_chance() {

            self.push_chance_nodes(node_idx, data);
            for action_idx in 0..node.num_actions() {
                let child_idx = node_idx + node.children_offset as usize + action_idx;
                self.build_tree(child_idx, &action_node.children[0].lock(), data);
            }
        
        } else {

            self.push_actions(node_idx, action_node, data);
            for action_idx in 0..node.num_actions() {
                let child_idx = node_idx + node.children_offset as usize + action_idx;
                self.build_tree(child_idx, &action_node.children[action_idx].lock(), data);
            }
        }
    }

    fn push_chance_nodes(&self, node_idx: usize, data: &mut BuildTreeData) {

        let mut node = self.node_arena[node_idx].lock();
        let flop_mask = self.board.flop_mask();

        // Turn not dealt.
        if !node.turn.is_dealt() {

            let skip_cards = &self.isomorphism_card_turn;
            let skip_mask: u64 = skip_cards.iter().map(|&c| 1 << c).sum();
            node.children_offset = (data.turn_idx - node_idx) as u32;

            // Push all possible turn cards.
            for t in 0..52 {
                // Check card is viable.
                if (1 << t) & (flop_mask | skip_mask) == 0 {
                    node.num_children += 1;
                    let mut child = node.children().last().unwrap().lock();
                    child.last_action = Action::Chance(t);
                    child.turn = Card(t);
                }
            }

            data.turn_idx += node.num_children as usize;
        
        } else {

            let turn_mask = flop_mask | node.turn.mask();
            let skip_cards = &self.isomorphism_card_river[node.turn.suit_u8() as usize];
            let skip_mask: u64 = skip_cards.iter().map(|&c| 1 << c).sum();
            node.children_offset = (data.river_idx - node_idx) as u32;

            // Push all possible river cards.
            for r in 0..52 {
                if (1 << r) & (turn_mask | skip_mask) == 0 {
                    node.num_children += 1;
                    let mut child = node.children().last().unwrap().lock();
                    child.last_action = Action::Chance(r);
                    child.turn = node.turn;
                    child.river = Card(r);
                }
            }

            data.river_idx += node.num_children as usize;
        }

        node.num_elements = node.cf_values_storage_player()
            .map_or(0, |player| self.num_private_hands(player)) as u32;

        data.num_storage_chance += node.num_elements as u64;
    }

    fn push_actions(&self, node_idx: usize, action_node: &ActionTreeNode, data: &mut BuildTreeData) {

        let mut node = self.node_arena[node_idx].lock();
        let street = match (node.turn, node.river) {
            (Card(0xFF), _) => Street::Flop,
            (_, Card(0xFF)) => Street::Turn,
            (_, _) => Street::River,
        };
        let base = match street {
            Street::Flop  => &mut data.flop_idx,
            Street::Turn  => &mut data.turn_idx,
            Street::River => &mut data.river_idx,
        };

        node.children_offset = (*base - node_idx) as u32;
        node.num_children = action_node.children.len() as u16;
        *base += node.num_children as usize;

        for (child, action) in node.children().iter().zip(action_node.actions.iter()) {
            let mut child = child.lock();
            child.last_action = *action;
            child.turn = node.turn;
            child.river = node.river;
        }

        let num_private_hands = self.num_private_hands(node.player as usize);
        node.num_elements = (node.num_actions() * num_private_hands) as u32;
        node.num_elements_ip = match node.last_action {
            Action::None | Action::Chance(_) => self.num_private_hands(PLAYER_IP as usize) as u16,
            _ => 0,
        };

        data.num_storage += node.num_elements as u64;
        data.num_storage_ip += node.num_elements_ip as u64;
    }

    // Number of nodes in game tree.
    fn count_nodes(&self) -> [u64; 3] {

        let (turn_coeff, river_coeff) = match (self.board.turn.0, self.board.river.0) {
            // Not dealt.
            (0xFF, _) => {
                let mut river_coeff = 0;
                // Can skip cards that have isomorphic pairs.
                let skip_cards = &self.isomorphism_card_turn;
                let flop_mask = self.board.flop_mask();
                let skip_mask: u64 = skip_cards.iter().map(|&c| 1 << c).sum();
                
                for t in 0..52 {
                    if (1 << t) & (flop_mask | skip_mask) == 0 {
                        // 48 because 52 cards - 3 flop - 1 turn.
                        river_coeff += 48 - self.isomorphism_card_river[t & 3].len();
                    }
                }
                
                (49 - self.isomorphism_card_turn.len(), river_coeff)
            },

            // Turn dealt.
            (turn, 0xFF) => (1, 48 - self.isomorphism_card_river[turn as usize & 3].len()),

            _ => (0, 1),
        };

        let num_action_nodes = count_action_nodes(&self.action_root.lock());

        [
            num_action_nodes[0],
            num_action_nodes[1] * turn_coeff as u64,
            num_action_nodes[2] * river_coeff as u64,
        ]

    }

    fn reset_bunching(&mut self) {
        self.bunching_num_dead = 0;
        self.bunching_num_combos = 0.0;
        self.bunching_arena = Vec::new();
        self.bunching_strength = Vec::new();
        self.bunching_num_flop = Default::default();
        self.bunching_num_turn = Default::default();
        self.bunching_num_river = Default::default();
        self.bunching_coeff_flop = Default::default();
        self.bunching_coeff_turn = Default::default();
        self.to_root();
    }
}

// Move to action module
fn count_action_nodes(node: &ActionTreeNode) -> [u64; 3] {
    
    let mut count = [0; 3];
    count_action_nodes_internal(node, 0, &mut count);
    
    if count[1] == 0 {
        count = [0, 0, count[0]];
    } else if count[2] == 0 {
        count = [0, count[0], count[1]];
    }
    count
}

fn count_action_nodes_internal(node: &ActionTreeNode, street: usize, count: &mut [u64; 3]) {
    count[street] += 1;
    if node.is_terminal() {
        // Base case.
    } else if node.is_chance() {
        count_action_nodes_internal(&node.children[0].lock(), street + 1, count)
    } else {
        for child in node.children.iter() {
            count_action_nodes_internal(&child.lock(), street, count)
        }
    }
}

fn iso_internal(
    iso_ref: &mut Vec<u8>,
    iso_card: &mut Vec<u8>,
    isomorphic_suit: &[Option<u8>; 4],
    mask: u64,
) {
    
    let push_card = iso_card.is_empty();
    let mut counter = 0;
    let mut idxs = [0; 52];

    for c in (0..52).into_iter().map(|c| Card(c)) {
        // If card dealt, skip.
        if mask & c.mask() != 0 {
            continue;
        }

        let suit = c.suit();
        if let Some(swap_suit) = isomorphic_suit[suit as usize] {
            
            let swap_card = c.swap_suit(swap_suit.into());
            iso_ref.push(idxs[swap_card.0 as usize]);
        
            if push_card {
                iso_card.push(swap_card.0);
            }
        
        } else { 
            idxs[c.0 as usize] = counter;
            counter += 1;
        }
    }
}

fn iso_swap_internal(
    swap_list: &mut [[Vec<(u16, u16)>; 2]; 4],
    reverse_table: &mut [usize],
    suits: [Suit; 2],
    hands: &[Vec<Hand>; 2],
) {

    let swap_list = &mut swap_list[suits[0] as usize];
    let swap = |card: Card| {
        if card.suit() == suits[0] {
            card.swap_suit(suits[1])
        } else if card.suit() == suits[1] {
            card.swap_suit(suits[0])
        } else {
            card
        }
    };

    for player in 0..2 {
        if !swap_list[player].is_empty() {
            continue;
        }

        reverse_table.fill(usize::MAX);
        let hands = &hands[player];

        for i in 0..hands.len() {
            reverse_table[Hand(hands[i].0, hands[i].1).idx()] = i;
        }

        for (i, &hand) in hands.iter().enumerate() {
            let c1 = swap(hand.0);
            let c2 = swap(hand.1);
            let idx = reverse_table[Hand(c1, c2).idx()];
            if i < idx {
                swap_list[player].push((i as u16, idx as u16));
            }
        }
    }
}

fn valid_idxs_street(hands: &[Vec<Hand>; 2], turn: Card, river: Card) -> [Vec<u16>; 2] {

    let mut idxs = [
        Vec::with_capacity(hands[0].len()),
        Vec::with_capacity(hands[1].len()),
    ];

    let mut used: u64 = 0;
    if turn.is_dealt() {
        used |= turn.mask();
    }
    if river.is_dealt() {
        used |= river.mask();
    }

    for player in 0..2 {
        idxs[player].extend(
            hands[player].iter().enumerate().filter_map(
                |(idx, &hand)| {
                    // Add index if hand doesn't conflict with board.
                    if hand.mask() & used == 0 {
                        Some(idx as u16)
                    } else {
                        None
                    }
        }));
        idxs[player].shrink_to_fit();
    }

    idxs
}


use std::{fmt::Debug, ops::DerefMut};
use crate::{Street, player::*, ConfigError, latch::Latch};
use super::{*, build_data::TreeBuildData, bets::BetSize};

// The action tree represents the possible sequences of actions taken by players throughout the game. 
// Each node in the action tree corresponds to a game state, and each edge represents an action taken by a player in that state.
#[derive(Default, Debug)]
pub struct ActionTree {

    pub config:         TreeConfig,
    
    pub added_lines:    Vec<Vec<Action>>,
    
    pub removed_lines:  Vec<Vec<Action>>,
    
    pub root:           Box<Latch<ActionTreeNode>>,
    
    pub history:        Vec<Action>,
}

#[derive(Default, Debug)]
pub struct ActionTreeNode {
    
    pub player: u8,

    pub street: Street,

    pub amount: i32,

    pub actions: Vec<Action>,

    pub children: Vec<Latch<ActionTreeNode>>,
}

impl ActionTreeNode {
    // At terminal nodes, no further actions are possible.
    pub fn is_terminal(&self) -> bool {
        self.player & PLAYER_TERMINAL_FLAG != 0
    }

    pub fn is_chance(&self) -> bool {
        self.player & PLAYER_CHANCE_FLAG != 0
    }

    fn num_nodes_internal(&self, total: &mut usize) {
        *total += 1;
        for child in self.children.iter() {
            child.lock().num_nodes_internal(total);
        }
    }

    fn print_node(&self, depth: usize, last_sibling: bool) {
        for _ in 0..depth {
            print!("│  ");
        }

        if last_sibling {
            print!("└──");
        } else {
            print!("├──");
        }

        println!("[Player: {:#x}, Street: {:?}, Amount: {}, Actions: {:?}]", self.player, self.street, self.amount, self.actions);

        let num_children = self.children.len();
        for (i, child) in self.children.iter().enumerate() {
            let is_last = i == num_children - 1;
            child.lock().print_node(depth + 1, is_last);
        }
    }
}

impl ActionTree {

    pub fn new(config: TreeConfig) -> Result<ActionTree, ConfigError> {
        
        // Ensure valid config values.
        config.verify()?;

        let mut tree = ActionTree {
            config,
            ..Default::default()
        };

        tree.init_build();
        Ok(tree)
    }

    pub fn init_build(&mut self) {

        // Initialise root node.
        let mut root = self.root.lock();
        *root = ActionTreeNode::default();
        root.street = self.config.initial_street;

        self.build_tree(&mut root, TreeBuildData::new(self.config.effective_stack));
    }

    pub fn build_tree(&self, node: &mut ActionTreeNode,data: TreeBuildData) {
        
        if node.is_terminal() { 
            // No further actions possible, base case.
            return;
        
        } else if node.is_chance() {
            
            // Move to next street.
            let next_street = match node.street {
                Street::Flop  => Street::Turn,
                Street::Turn  => Street::River,
                Street::River => unreachable!("River is the last street."),
            };

            let next_player = match (data.all_in, node.street) {
                (false, _) => PLAYER_OOP,
                (true, Street::Flop) => PLAYER_CHANCE_FLAG | PLAYER_CHANCE,
                (true, _) => PLAYER_TERMINAL_FLAG,
            };

            node.actions.push(Action::Chance(0));
            node.children.push(Latch::new(ActionTreeNode {
                player: next_player,
                street: next_street,
                amount: node.amount,
                ..Default::default()
            }));

            self.build_tree(&mut node.children[0].lock(), data.next(0, Action::Chance(0)));
        
        } else {

            self.push_actions(node, &data);
            for (action, child) in node.actions.iter().zip(node.children.iter()) {
                self.build_tree(&mut child.lock(), data.next(node.player, *action));
            }
        } 
    } 

    // All possible actions pushed to node.
    fn push_actions(&self, node: &mut ActionTreeNode, data: &TreeBuildData) {
        
        let player = node.player;
        let opp = player ^ 1;
        
        let player_stack = data.stacks[player as usize];
        let opp_stack = data.stacks[opp as usize];

        let prev_amount = data.last_amount;
        let to_call = player_stack - opp_stack;

        let pot = self.config.starting_pot + 2 * (node.amount + to_call);
        let max_amount = opp_stack + prev_amount;
        let min_amount = (prev_amount + to_call).clamp(1, max_amount);

        // Stack-to-pot ratio after call.
        let spr_after_call = opp_stack as f64 / pot as f64;
        // Calculate goeometric bet sizing.
        let calc_geometric = |n_streets: i32, max_ratio: f64| {
            let ratio = ((2.0 * spr_after_call + 1.0).powf(1.0 / n_streets as f64) - 1.0) / 2.0;
            (pot as f64 * ratio.min(max_ratio)).round() as i32
        };

        // Get bet size candidates.
        let (sizes, n_streets_left) = match node.street {
            Street::Flop =>  (&self.config.bet_sizings.flop,  3),
            Street::Turn =>  (&self.config.bet_sizings.turn,  2),
            Street::River => (&self.config.bet_sizings.river, 1),
        };

        let mut actions = Vec::new();

        // Not facing a bet.
        if matches!(data.last_action, Action::None | Action::Check | Action::Chance(_)) {

            // Check.
            actions.push(Action::Check);

            // Add available bet sizes to actions.
            for &bet_size in sizes[player as usize].bet.iter() {
                match bet_size {

                    BetSize::Absolute(amount) => actions.push(Action::Bet(amount)),

                    BetSize::PotScaled(ratio) => {
                        let amount = (pot as f64 * ratio).round() as i32;
                        actions.push(Action::Bet(amount));
                    },

                    BetSize::PrevScaled(_) => panic!("Can't scale non-existent previous bet."),

                    BetSize::Geometric(n_streets, max_ratio) => {
                        let n_streets = match n_streets {
                            0 => n_streets_left,
                            _ => n_streets,
                        };
                        let amount = calc_geometric(n_streets, max_ratio);
                        actions.push(Action::Bet(amount));
                    },

                    BetSize::AllIn => actions.push(Action::AllIn(max_amount)),
                }
                
            }
            
            // Add allin if threshold as proportion of pot reached.
            if max_amount <= (pot as f64 * self.config.add_all_in_threshold).round() as i32 {
                actions.push(Action::AllIn(max_amount));
            }

        // Facing bet.
        } else {

            // Fold.
            actions.push(Action::Fold);

            // Call.
            actions.push(Action::Call);

            if !data.all_in {

                // Can raise a non allin bet.
                for &bet_size in sizes[player as usize].raise.iter() {
                    match bet_size {

                        // TODO: Maybe change to additive.
                        BetSize::Absolute(amount) => actions.push(Action::Raise(amount)),

                        BetSize::PotScaled(ratio) => {
                            let amount = prev_amount + (pot as f64 * ratio).round() as i32; 
                            actions.push(Action::Raise(amount));
                        },

                        BetSize::PrevScaled(ratio) => {
                            let amount = (prev_amount as f64 * ratio).round() as i32;
                            actions.push(Action::Raise(amount));
                        },


                        BetSize::Geometric(n_streets, max_ratio) => {
                            let n_streets = match n_streets {
                                0 => (n_streets_left - data.num_bets + 1).max(1),
                                _ => (n_streets - data.num_bets + 1).max(1),
                            };
                            let amount = calc_geometric(n_streets, max_ratio);
                            actions.push(Action::Raise(prev_amount + amount));
                        },

                        BetSize::AllIn => actions.push(Action::AllIn(max_amount)),
                    }
                }

                // All in if theshold reached.
                let threshold = pot as f64 * self.config.add_all_in_threshold;
                if max_amount <= prev_amount + threshold.round() as i32 {
                    actions.push(Action::AllIn(max_amount));
                }
            }
        }

        let breaks_threshold = |amount: i32| {
            let diff = amount - prev_amount;
            let new_pot = pot + 2 * diff;
            let threshold = (new_pot as f64 * self.config.force_all_in_threshold).round() as i32;
            max_amount <= threshold + amount
        };

        // Check if betting amounts break the force all-in threshold.
        // Otherwise clamp bets between min and max amounts.
        for action in actions.iter_mut() {
            match *action {

                Action::Bet(amount) => {
                    let clamped = amount.clamp(min_amount, max_amount);
                    if breaks_threshold(clamped) {
                        *action = Action::AllIn(max_amount);
                    } else if clamped != amount {
                        *action = Action::Bet(clamped);
                    }
                },

                Action::Raise(amount) => {
                    let clamped = amount.clamp(min_amount, max_amount);
                    if breaks_threshold(clamped) {
                        *action = Action::AllIn(max_amount);
                    } else if clamped != amount {
                        *action = Action::Raise(clamped);
                    }
                },
                
                _ => {},
            }
        }

        actions.sort_unstable();
        actions.dedup(); // Remove duplicates.

        // TODO: merge close bets?

        let player_after_call = match node.street {
            Street::River => PLAYER_TERMINAL_FLAG, // End of hand.
            _ => PLAYER_CHANCE_FLAG | player,
        };

        let player_after_check = match player {
            PLAYER_OOP => opp,
            _ => player_after_call,
        };

        for action in actions {

            let mut amount = node.amount;
            let next_player = match action {

                Action::Fold => PLAYER_FOLD_FLAG | player,

                Action::Check => player_after_check,

                Action::Call => {
                    amount += to_call;
                    player_after_call
                },
                
                Action::Bet(_) | Action::Raise(_) | Action::AllIn(_) => {
                    amount += to_call;
                    opp
                },
                
                _ => unreachable!("Invalid action."),
            };

            node.actions.push(action);
            node.children.push(Latch::new(ActionTreeNode {
                player: next_player,
                street: node.street,
                amount,
                ..Default::default()
            }));
        }

        node.actions.shrink_to_fit();
        node.children.shrink_to_fit();
    }

    pub fn num_nodes(&self) -> usize {
        let mut total = 0;
        self.root.lock().num_nodes_internal(&mut total);
        total
    }

    // Print nodes in a tree-like structure.
    pub fn print_nodes(&self) {
        self.root.lock().print_node(0, false);
    }

    // Search for invalid terminal nodes, should return empty vec.
    pub fn invalid_terminals(&self) -> Vec<Vec<Action>> {
        let mut ret = Vec::new();
        let mut line = Vec::new();
        Self::invalid_terminals_internal(&self.root.lock(), &mut ret, &mut line);
        ret
    }

    fn invalid_terminals_internal(
        node: &ActionTreeNode,
        result: &mut Vec<Vec<Action>>,
        line: &mut Vec<Action>,
    ) {
        if node.is_terminal() {
            // Base case.
        } else if node.children.is_empty() {
            // Final node not registered as terminal is invalid.
            result.push(line.clone());
        } else if node.is_chance() {
            Self::invalid_terminals_internal(&node.children[0].lock(), result, line)
        } else {
            for (&action, child) in node.actions.iter().zip(node.children.iter()) {
                line.push(action);
                Self::invalid_terminals_internal(&child.lock(), result, line);
                line.pop();
            }
        }
    }
}
use std::sync::Mutex;

use poker::card::Card;
use thiserror::Error;
use crate::bet_size::{Bets, BetParseError, BetSize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Street {
    #[default]
    Flop,
    Turn,
    River,
}

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Action {
    #[default]
    None,
    Fold,
    Check,
    Call,
    Bet(u32),
    Raise(u32),
    AllIn(u32),
    Chance(Card),
}

#[derive(Clone, Default, Debug)]
pub struct TreeConfig {

    pub initial_street: Street,
    
    pub starting_pot: u32,
    
    pub effective_stack: u32,
    
    pub rake: f32,
    
    pub rake_cap: f32,
    
    pub bet_sizings: Bets,
    
    pub add_all_in_threshold: f32,

    pub force_all_in_threshold: f32,
}

#[derive(Error, Debug)]
pub enum TreeConfigError {
    #[error("Input number for {0} is invalid.")]
    InvalidNumber(String),
    #[error("Bet sizing error: {0}")]
    BetSizing(#[from] BetParseError),
}

impl TreeConfig {

    pub fn verify(&self) -> Result<(), TreeConfigError> {

        if self.rake < 0.0 || self.rake > 1.0 {
            return Err(TreeConfigError::InvalidNumber("Rake".to_string()));
        }
        if self.rake_cap < 0.0 {
            return Err(TreeConfigError::InvalidNumber("Rake cap".to_string()));
        }
        if self.add_all_in_threshold < 0.0 {
            return Err(TreeConfigError::InvalidNumber("Add all in threshold".to_string()));
        }
        if self.force_all_in_threshold < 0.0 {
            return Err(TreeConfigError::InvalidNumber("Force all in threshold".to_string()));
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct ActionTree {

    config:         TreeConfig,
    
    added_lines:    Vec<Vec<Action>>,
    
    removed_lines:  Vec<Vec<Action>>,
    
    root:           Box<Mutex<ActionTreeNode>>,
    
    history:        Vec<Action>,
}

#[derive(Debug, Default, Clone)]
pub struct BuildTreeData {

    last_action: Action,
    
    last_amount: u32,
    
    n_bets: u32,
    
    allin: bool,
    
    oop_call: bool, 
    
    stacks: [u32; 2],
}

impl BuildTreeData {

    fn new(stack_size: u32) -> BuildTreeData {
        BuildTreeData {
            stacks: [stack_size, stack_size], 
            ..Default::default()
        }
    }

    fn next(&self, player: u8, action: Action) -> BuildTreeData {
        
        let mut n_bets = self.n_bets;
        let mut allin = self.allin;
        let mut stacks = self.stacks;
        let mut last_amount = self.last_amount;
        let mut oop_call = self.oop_call;

        match action {
            
            Action::Check => oop_call = false,

            Action::Call => {
                n_bets = 0;
                oop_call = player == PLAYER_OOP;
                stacks[player as usize] = stacks[player as usize ^ 1];
                last_amount = 0;
            },

            Action::Bet(n) | Action::Raise(n) | Action::AllIn(n) => {
                let to_call = stacks[player as usize] - stacks[player as usize ^ 1];
                n_bets += 1;
                allin = matches!(action, Action::AllIn(_));
                stacks[player as usize] -= n - last_amount + to_call;
                last_amount = n;
            }, 

            _ => {},
        }
        
        BuildTreeData {
            last_action: action,
            n_bets,
            allin,
            stacks,
            last_amount,
            oop_call,
        }
    }
}

const PLAYER_OOP: u8            = 0b0000_0000;
const PLAYER_IP: u8             = 0b0000_0001;
const PLAYER_CHANCE: u8         = 0b0000_0010;
const PLAYER_MASK: u8           = 0b0000_0011;
const PLAYER_CHANCE_FLAG: u8    = 0b0000_0100;
const PLAYER_TERMINAL_FLAG: u8  = 0b0000_1000;
const PLAYER_FOLD_FLAG: u8      = 0b0001_1000;


#[derive(Debug, Default)]
pub struct ActionTreeNode {
    
    pub player: u8,

    pub street: Street,

    pub amount: u32,

    pub actions: Vec<Action>,

    pub children: Vec<Mutex<ActionTreeNode>>,
}

impl ActionTreeNode {

    pub fn is_terminal(&self) -> bool {
        self.player & PLAYER_TERMINAL_FLAG != 0
    }

    pub fn is_chance(&self) -> bool {
        self.player & PLAYER_CHANCE_FLAG != 0
    }

}

impl ActionTree {

    pub fn new(config: TreeConfig) -> Result<ActionTree, TreeConfigError> {
        
        config.verify()?;

        let mut tree = ActionTree {
            config,
            ..Default::default()
        };

        tree.init_build();
        Ok(tree)
    }

    pub fn init_build(&mut self) {

        let mut root = self.root.lock().unwrap();
        *root = ActionTreeNode::default();
        root.street = self.config.initial_street;

        self.build_tree(&mut root, BuildTreeData::new(self.config.effective_stack));
    }

    pub fn build_tree(&self, node: &mut ActionTreeNode,data: BuildTreeData) {

        if node.is_terminal() {
            return;
        
        } else if node.is_chance() {
            
            // Move to next street.
            let next_street = match node.street {
                Street::Flop  => Street::Turn,
                Street::Turn  => Street::River,
                Street::River => unreachable!("River is the last street."),
            };

            let next_player = match (data.allin, node.street) {
                (false, _) => PLAYER_OOP,
                (true, Street::Flop) => PLAYER_CHANCE_FLAG | PLAYER_CHANCE,
                (true, _) => PLAYER_TERMINAL_FLAG,
            };

            node.actions.push(Action::Chance(Card::default()));
            node.children.push(Mutex::new(ActionTreeNode {
                player: next_player,
                street: next_street,
                amount: node.amount,
                ..Default::default()
            }));            

            self.build_tree(&mut node.children[0].lock().unwrap(), data.next(0, Action::Chance(Card::default())));
        
        } else {

            self.push_actions(node, &data);
            for (action, child) in node.actions.iter().zip(node.children.iter()) {
                self.build_tree(&mut child.lock().unwrap(), data.next(node.player, *action));
            }
        } 
    } 

    // All possible actions pushed to node.
    fn push_actions(&self, node: &mut ActionTreeNode, data: &BuildTreeData) {
        
        let player = node.player;
        let player_stack = data.stacks[player as usize];
        
        let opp = player ^ 1;
        let opp_stack = data.stacks[opp as usize];

        let prev_amount = data.last_amount;
        let to_call = player_stack - opp_stack;

        let pot = self.config.starting_pot + 2 * (node.amount + to_call);
        let max_amount = opp_stack + prev_amount;
        let min_amount = (prev_amount + to_call).clamp(1, max_amount);

        let spr_after_call = opp_stack as f32 / pot as f32;
        let fn_geometric = |n_streets: u32, max_ratio: f32| {
            let ratio = ((2.0 * spr_after_call + 1.0).powf(1.0 / n_streets as f32) - 1.0) / 2.0;
            (pot as f32 * ratio.min(max_ratio)).round() as u32
        };

        let (sizes, n_streets_left) = match node.street {
            Street::Flop =>  (&self.config.bet_sizings.flop,  3),
            Street::Turn =>  (&self.config.bet_sizings.turn,  2),
            Street::River => (&self.config.bet_sizings.river, 1),
        };

        let mut actions = Vec::new();

        if matches!(data.last_action, Action::None | Action::Check | Action::Chance(_)) {

            // Check.
            actions.push(Action::Check);

            // Bet.
            for bet_size in sizes[player as usize].bet.iter() {
                match bet_size {

                    BetSize::Absolute(amount) => actions.push(Action::Bet(*amount)),

                    BetSize::PotScaled(ratio) => {
                        let amount = (pot as f32 * ratio).round() as u32;
                        actions.push(Action::Bet(amount));
                    },

                    BetSize::PrevScaled(_) => panic!("Can't scale non-existent previous bet."),

                    BetSize::Geometric(n_streets, max_ratio) => {
                        let n_streets = match n_streets {
                            0 => n_streets_left,
                            _ => *n_streets,
                        };
                        let amount = fn_geometric(n_streets, *max_ratio);
                        actions.push(Action::Bet(amount));
                    },

                    BetSize::AllIn => actions.push(Action::AllIn(max_amount)),
                }
                
            }
            
            // All in.
            if max_amount <= (pot as f32 * self.config.add_all_in_threshold).round() as u32 {
                actions.push(Action::AllIn(max_amount));
            }
        } else {

            // Fold.
            actions.push(Action::Fold);

            // Call.
            actions.push(Action::Call);

            if !data.allin {

                // Raise.
                for bet_size in sizes[player as usize].raise.iter() {
                    match bet_size {

                        BetSize::PotScaled(ratio) => {
                            let amount = prev_amount + (pot as f32 * ratio).round() as u32; 
                            actions.push(Action::Raise(amount));
                        },

                        BetSize::PrevScaled(ratio) => {
                            let amount = (prev_amount as f32 * ratio).round() as u32;
                            actions.push(Action::Raise(amount));
                        },

                        // TODO: Maybe change.
                        BetSize::Absolute(amount) => actions.push(Action::Raise(*amount)),

                        BetSize::Geometric(n_streets, max_ratio) => {
                            let n_streets = match n_streets {
                                0 => (n_streets_left - data.n_bets + 1).max(1),
                                _ => (*n_streets - data.n_bets + 1).max(1),
                            };
                            let amount = fn_geometric(n_streets, *max_ratio);
                            actions.push(Action::Raise(prev_amount + amount));
                        },

                        BetSize::AllIn => actions.push(Action::AllIn(max_amount)),
                    }
                }

                // All in.
                let threshold = pot as f32 * self.config.add_all_in_threshold;
                if max_amount <= prev_amount + threshold.round() as u32 {
                    actions.push(Action::AllIn(max_amount));
                }
            }
        }

        let breaks_threshold = |amount: u32| {
            let diff = amount - prev_amount;
            let new_pot = pot + 2 * diff;
            let threshold = (new_pot as f32 * self.config.force_all_in_threshold).round() as u32;
            max_amount <= threshold + amount
        };

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
        actions.dedup();

        // TODO: merge close bets?

        let player_after_call = match node.street {
            Street::River => PLAYER_TERMINAL_FLAG,
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
            node.children.push(Mutex::new(ActionTreeNode {
                player: next_player,
                street: node.street,
                amount,
                ..Default::default()
            }));
        }

        node.actions.shrink_to_fit();
        node.children.shrink_to_fit();
    }
}
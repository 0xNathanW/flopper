use std::fmt::Debug;
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

    // If node is chance, return actions after chance event.
    pub fn available_actions(&self) -> &[Action] {
        &self.current_node_ex_chance().actions
    }

    pub fn play(&mut self, action: Action) -> Result<(), String>{
        let node = self.current_node_ex_chance();
        if !node.actions.contains(&action) {
            return Err(format!("Invalid action: {:?}.", action));
        }

        self.history.push(action);
        Ok(())
    }

    pub fn apply_history(&mut self, history: &[Action]) -> Result<(), String> {
        self.history.clear();
        for &action in history {
            self.play(action)?;
        }
        Ok(())
    }

    pub fn add_line(&mut self, line: &[Action]) -> Result<(), String> {
        
        let removed_idx = self.removed_lines.iter().position(|l| l == line);
        let is_replaced = self.add_line_internal(
            &mut self.root.lock(),
            line,
            &TreeBuildData::new(self.config.effective_stack),
            removed_idx.is_some(),
        )?;

        if let Some(idx) = removed_idx {
            self.removed_lines.remove(idx);
        } else {
            let mut line = line.to_vec();
            if is_replaced {
                if let Some(&Action::Bet(amount) | &Action::Raise(amount)) = line.last() {
                    *line.last_mut().unwrap() = Action::AllIn(amount);
                }
            }
            self.added_lines.push(line);
        }
        Ok(())
    }

    pub fn remove_line(&mut self, line: &[Action]) -> Result<(), String> {
        
        Self::remove_line_internal(&mut self.root.lock(), line)?;
        let was_added = self.added_lines.iter().any(|l| l == line);
        self.added_lines.retain(|l| !l.starts_with(line));
        self.removed_lines.retain(|l| !l.starts_with(line));

        if !was_added {
            self.removed_lines.push(line.to_vec());
        }
        if self.history.starts_with(line) {
            self.history.truncate(line.len() - 1);
        }

        Ok(())
    }

    pub fn remove_current_node(&mut self) -> Result<(), String> {
        let history = self.history.clone();
        self.remove_line(&history)
    }

    pub fn add_action(&mut self, action: Action) -> Result<(), String> {
        let mut action_line = self.history.clone();
        action_line.push(action);
        self.add_line(&action_line)
    }

    pub fn is_terminal_node(&self) -> bool {
        self.current_node_ex_chance().is_terminal()
    }

    pub fn is_chance_node(&self) -> bool {
        self.current_node().is_chance() && !self.is_terminal_node()
    }

    pub fn total_bet_amount(&self) -> [i32; 2] {
        let info = TreeBuildData::new(self.config.effective_stack);
        self.total_bet_amount_internal(&self.root.lock(), &self.history, &info)
    }
}

impl ActionTree {

    pub fn current_node(&self) -> &ActionTreeNode {
        unsafe {
            let mut node = &*self.root.lock() as *const ActionTreeNode;
            for action in &self.history {
                while (*node).is_chance() {
                    node = &*(*node).children[0].lock();
                }
                let idx = (*node).actions.iter().position(|a| a == action).unwrap();
                node = &*(*node).children[idx].lock();
            }
            &*node
        }
    }

    fn current_node_ex_chance(&self) -> &ActionTreeNode {
        unsafe {
            let mut node = self.current_node() as *const ActionTreeNode;
            while (*node).is_chance() {
                node = &*(*node).children[0].lock();
            }
            &*node
        }
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

    fn total_bet_amount_internal(
        &self,
        node: &ActionTreeNode,
        line: &[Action],
        info: &TreeBuildData,
    ) -> [i32; 2] {
        
        if line.is_empty() || node.is_terminal() {
            let stack = self.config.effective_stack;
            return [stack - info.stacks[0], stack - info.stacks[1]];
        }

        if node.is_chance() {
            return self.total_bet_amount_internal(&node.children[0].lock(), line, info);
        }

        let action = line[0];
        let i = node.actions.binary_search(&action).expect(
            "Action not found in node.  Should have been caught by invalid_terminals().",
        );

        let nxt_info = info.next(node.player, action);
        self.total_bet_amount_internal(&node.children[i].lock(), &line[1..], &nxt_info)
    }

    fn add_line_internal(
        &self,
        node: &mut ActionTreeNode,
        line: &[Action],
        info: &TreeBuildData,
        was_removed: bool,
    ) -> Result<bool, String> {

        if line.is_empty() {
            return Err("Empty line.".to_string());
        }

        if node.is_terminal() {
            return Err("Terminal node.".to_string());
        }

        if node.is_chance() {
            return self.add_line_internal(
                &mut node.children[0].lock(), 
                line, 
                &info.next(0, Action::Chance(0)), 
                was_removed
            );
        }

        let action = line[0];
        let search_result = node.actions.binary_search(&action);
        let player = node.player;
        let opp = node.player ^ 1;

        if line.len() > 1 {
            
            if search_result.is_err() {
                return Err("Non-Existant action.".to_string());
            }

            return self.add_line_internal(
                &mut node.children[search_result.unwrap()].lock(), 
                &line[1..], 
                &info.next(player, action), 
                was_removed
            );
        }

        if search_result.is_ok() {
            return Err("Action already exists.".to_string());
        }

        let is_bet_action = matches!(action, Action::Bet(_) | Action::Raise(_) | Action::AllIn(_));
        if info.all_in && is_bet_action {
            return Err("Bet after all-in reached.".to_string());
        }

        let player_stack = info.stacks[player as usize];
        let opp_stack = info.stacks[opp as usize];
        let prev_amount = info.last_amount;
        let to_call = player_stack - opp_stack;

        let max_amount = opp_stack + prev_amount;
        let min_amount = (prev_amount + to_call).clamp(1, max_amount);

        let mut is_replaced = false;
        let action = match action {
            Action::Bet(amount) | Action::Raise(amount) if amount == max_amount => {
                is_replaced = true;
                Action::AllIn(amount)
            },
            _ => action,
        };

        let is_valid_bet = match action {
            
            Action::Bet(amount) if amount >= min_amount && amount < max_amount => {
                matches!(info.last_action, Action::None | Action::Check | Action::Chance(_))
            },

            Action::Raise(amount) if amount >= min_amount && amount < max_amount => {
                matches!(info.last_action, Action::Bet(_) | Action::Raise(_))
            },

            Action::AllIn(amount) => amount == max_amount,

            _ => false,
        };

        if !was_removed && !is_valid_bet {
            return Err("Invalid bet.".to_string());
        }

        let player_after_call = match node.street {
            Street::River => PLAYER_TERMINAL_FLAG, // End of hand.
            _ => PLAYER_CHANCE_FLAG | player,
        };
        
        let player_after_check = match player {
            PLAYER_OOP => opp,
            _ => player_after_call,
        };

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

        let idx = search_result.unwrap_err();
        node.actions.insert(idx, action);
        node.children.insert(idx, Latch::new(ActionTreeNode {
            player: next_player,
            street: node.street,
            amount,
            ..Default::default()
        }));

        node.actions.shrink_to_fit();
        node.children.shrink_to_fit();

        self.build_tree(&mut node.children[idx].lock(), info.next(player, action));

        Ok(is_replaced)
    }

    fn remove_line_internal(node: &mut ActionTreeNode, line: &[Action]) -> Result<(), String> {

        if line.is_empty() {
            return Err("Empty line.".to_string());
        }

        if node.is_terminal() {
            return Err("Terminal node.".to_string());
        }

        if node.is_chance() {
            return Self::remove_line_internal(&mut node.children[0].lock(), line);
        }

        let action = line[0];
        let search_result = node.actions.binary_search(&action);
        if search_result.is_err() {
            return Err("Non-Existant action.".to_string());
        }

        if line.len() > 1 {
            return Self::remove_line_internal(&mut node.children[search_result.unwrap()].lock(), &line[1..]);
        }

        let idx = search_result.unwrap();
        node.actions.remove(idx);
        node.children.remove(idx);

        node.actions.shrink_to_fit();
        node.children.shrink_to_fit();

        Ok(())
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
}
use super::{Action};
use crate::player::*;

// Used as input in recursive function to build action tree.
#[derive(Debug, Default, Clone)]
pub struct TreeBuildData {

    pub last_action: Action,
    
    pub last_amount: u32,
    
    pub num_bets: u32,
    
    pub all_in: bool, 
    
    pub oop_call: bool, 
    
    pub stacks: [u32; 2],
}

impl TreeBuildData {

    pub fn new(stack_size: u32) -> TreeBuildData {
        TreeBuildData {
            stacks: [stack_size, stack_size], 
            ..Default::default()
        }
    }

    pub fn next(&self, player: u8, action: Action) -> TreeBuildData {
        
        let mut num_bets    = self.num_bets;
        let mut all_in     = self.all_in;
        let mut stacks = self.stacks;
        let mut last_amount = self.last_amount;
        let mut oop_call   = self.oop_call;

        match action {
            // Can't call following a check.
            Action::Check => oop_call = false,

            Action::Call => {
                num_bets = 0;
                oop_call = player == PLAYER_OOP;
                // Effective stack equal after call.
                stacks[player as usize] = stacks[player as usize ^ 1];
                last_amount = 0;
            },

            Action::Bet(n) | Action::Raise(n) | Action::AllIn(n) => {
                // Amount to call is difference between stacks.
                let to_call = stacks[player as usize] - stacks[player as usize ^ 1];
                num_bets += 1;
                // Set all_in flag.
                all_in = matches!(action, Action::AllIn(_));
                // Minus bet amount, last bet (in case of raise) to player stack.
                stacks[player as usize] -= n - last_amount + to_call;
                last_amount = n;
            }, 

            _ => {},
        }
        
        TreeBuildData {
            last_action: action,
            num_bets,
            all_in,
            stacks,
            last_amount,
            oop_call,
        }
    }
}
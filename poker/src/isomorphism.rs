use crate::card::{Suit, SUITS};
use std::collections::HashSet;

pub fn valid_suit_permutations(suits_on_board: &HashSet<Suit>) -> Vec<[Suit; 4]> {
    let mut result = vec![SUITS];
    
    let free_suits: Vec<Suit> = SUITS.iter()
        .filter(|suit| !suits_on_board.contains(suit))
        .cloned()
        .collect();
    
    let free_positions: Vec<usize> = (0..4)
        .filter(|&i| !suits_on_board.contains(&SUITS[i]))
        .collect();
    
    // Base case.
    if free_suits.len() <= 1 {
        return result;
    }
    
    let mut free_suit_perms = Vec::new();
    permute_array(&free_suits, &mut free_suit_perms);
    
    let base_perm = result[0];
    result.clear();
    
    for perm in free_suit_perms {
        let mut new_perm = base_perm;
        
        for (i, pos) in free_positions.iter().enumerate() {
            new_perm[*pos] = perm[i];
        }
        
        result.push(new_perm);
    }
    
    result
}

fn permute_array<T: Clone>(arr: &[T], result: &mut Vec<Vec<T>>) {
    fn permute_helper<T: Clone>(arr: &mut [T], start: usize, result: &mut Vec<Vec<T>>) {
        
        if start == arr.len() {
            result.push(arr.to_vec());
            return;
        }
        
        for i in start..arr.len() {
            arr.swap(start, i);
            permute_helper(arr, start + 1, result);
            arr.swap(start, i);
        }
    }
    
    let mut arr_copy = arr.to_vec();
    permute_helper(&mut arr_copy, 0, result);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_no_board_constraints() {
        let suits = HashSet::new();
        let permutations = valid_suit_permutations(&suits);
        assert_eq!(permutations.len(), 24);
        let unique: HashSet<_> = permutations.iter().cloned().collect();
        assert_eq!(unique.len(), 24);
    }
    
    #[test]
    fn test_one_suit_on_board() {
        let mut suits = HashSet::new();
        suits.insert(Suit::Hearts);
        let permutations = valid_suit_permutations(&suits);
        
        // For one locked suit, should have 6 permutations (3!)
        assert_eq!(permutations.len(), 6);
        
        // Verify hearts is always in its original position.
        for perm in &permutations {
            assert_eq!(perm[1], Suit::Hearts);
        }
    }
    
    #[test]
    fn test_two_suits_on_board() {
        let mut suits = HashSet::new();
        suits.insert(Suit::Hearts);
        suits.insert(Suit::Spades);
        
        let permutations = valid_suit_permutations(&suits);
        
        // For two locked suits, should have 2 permutations (2!)
        assert_eq!(permutations.len(), 2);
        
        // Verify locked suits are in original positions.
        for perm in &permutations {
            assert_eq!(perm[0], Suit::Spades);
            assert_eq!(perm[1], Suit::Hearts);
        }
    }
    
    #[test]
    fn test_three_suits_on_board() {
        let mut suits = HashSet::new();
        suits.insert(Suit::Hearts);
        suits.insert(Suit::Spades);
        suits.insert(Suit::Diamonds);
        
        let permutations = valid_suit_permutations(&suits);
        
        assert_eq!(permutations.len(), 1);
        assert_eq!(permutations[0], [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs]);
    }
}
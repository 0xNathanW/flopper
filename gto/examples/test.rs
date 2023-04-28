use gto::{bet_size::{BetSizings, Bets}, tree::{TreeConfig, Street, ActionTree}};
// use poker::{range::Range, card::Card};

fn main() {

    // let oop_range = Range::from_str("66+,A8s+,A4s-A5s,AJo+,K9s+,KQo,QTs+,JTs,96s+,85s+,75s+,65s,54s");
    // let ip_range = Range::from_str("22-QQ,A2s-AQs,ATo+,K5s+,KJo+,Q8s+,J8s+,T7s+,96s+,86s+,75s+,64s+,53s+");
    // let board = Card::vec_from_str("Td 9d 6h Qc");
    
    let sizings = BetSizings::from_str("60%, e, a", "2.5x").unwrap(); 
    println!("{:#?}", sizings);
    let bets = Bets {
        flop: [sizings.clone(), sizings.clone()],
        turn: [sizings.clone(), sizings.clone()],
        river: [sizings.clone(), sizings.clone()],
    };

    let tree_config = TreeConfig {
        initial_street:         Street::Turn,
        starting_pot:           200,
        effective_stack:        900,
        rake:                   0.0,
        rake_cap:               0.0,
        bet_sizings:            bets,
        add_all_in_threshold:   1.5,
        force_all_in_threshold: 0.15,
    };

    let tree = ActionTree::new(tree_config).unwrap();
    println!("{:#?}", tree);
    
}
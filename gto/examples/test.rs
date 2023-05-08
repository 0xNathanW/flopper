use gto::game::Game;
use gto::{Street, solve::solve};
use gto::action::{BetSizingsStreet, ActionTree, TreeConfig, BetSizings};
use gto::postflop::PostFlopGame;
use poker::Board;
use poker::range::Range;

fn main() {

    let oop_range = Range::from_str("66+,A8s+,A4s-A5s,AJo+,K9s+,KQo,QTs+,JTs,96s+,85s+,75s+,65s,54s").unwrap();
    let ip_range = Range::from_str("22-QQ,A2s-AQs,ATo+,K5s+,KJo+,Q8s+,J8s+,T7s+,96s+,86s+,75s+,64s+,53s+").unwrap();
    let board = Board::from_str("Td 9d 6h Qc").unwrap();
    
    let sizings = BetSizingsStreet::from_str("60%, e, a", "2.5x").unwrap(); 

    let bets = BetSizings {
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
    let mut game = PostFlopGame::new([oop_range, ip_range], board,tree).unwrap();
    let root = game.root();
    
    // let (uncompressed, compressed) = game.memory_usage();
    // println!("Uncompressed mem usage {} bytes", uncompressed);
    // println!("Compressed mem usage {} bytes", compressed);
    
    game.allocate_memory(false);

    let target_exploitability = game.tree_config().starting_pot as f32 * 0.005;
    solve(&mut game, 1000, target_exploitability);
}
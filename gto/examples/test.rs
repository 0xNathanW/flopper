use gto::{Street, solve::solve};
use gto::action::{BetSizingsStreet, ActionTree, TreeConfig, BetSizings, Action};
use gto::postflop::PostFlopGame;
use poker::{Board, Card};
use poker::range::Range;

fn main() {

    let oop_range = Range::from_str("KK+").unwrap();
    let ip_range = Range::from_str("22+").unwrap();
    let board = Board::from_str("Td 9d 6h").unwrap();
    
    let sizings = BetSizingsStreet::from_str("50%", "50%").unwrap();

    let bets = BetSizings {
        flop: [sizings.clone(), sizings.clone()],
        turn: [sizings.clone(), sizings.clone()],
        river: [sizings.clone(), sizings.clone()],
    };

    let tree_config = TreeConfig {
        initial_street:         Street::Flop,
        starting_pot:           40,
        effective_stack:        100,
        rake:                   0.0,
        rake_cap:               3.0,
        bet_sizings:            bets,
        add_all_in_threshold:   0.0,
        force_all_in_threshold: 0.0,
    };

    let mut tree = ActionTree::new(tree_config).unwrap();

    tree.apply_history(&[Action::Bet(20), Action::Raise(60), Action::AllIn(100)]).unwrap();
    let current_node = tree.current_node();
    println!("{:?}", current_node);
    let mut game = PostFlopGame::new([oop_range, ip_range], board,tree).unwrap();

    let (uncompressed, compressed) = game.memory_usage();
    println!("Uncompressed mem usage {} bytes", uncompressed);
    println!("Compressed mem usage {} bytes", compressed);
    
    // game.allocate_memory(false);

    // let target_exploitability = game.tree_config().starting_pot as f32 * 0.005;
    // let e = solve(&mut game, 1000, target_exploitability);

    // println!("Exploitability: {}", e);

    // game.cache_normalised_weights();
    // let equity = game.equity(0); // `0` means OOP player
    // let ev = game.expected_values(0);
    // println!("Equity of oop_hands[0]: {:.2}%", 100.0 * equity[0]);
    // println!("EV of oop_hands[0]: {:.2}", ev[0]);

    // // get equity and EV of whole hand
    // let weights = game.normalised_weights(0);
    // let average_equity = average(&equity, weights);
    // let average_ev = average(&ev, weights);
    // println!("Average equity: {:.2}%", 100.0 * average_equity);
    // println!("Average EV: {:.2}", average_ev);

    // let actions = game.available_actions();
    // println!("{:?}", actions);

    // // play `Bet(120)`
    // game.play(1);

    // // get available actions (IP)
    // let actions = game.available_actions();
    // println!("{:?}", actions);

    // // confirm that IP does not fold the nut straight
    // let ip_cards = game.hands(1);
    // let strategy = game.strategy();
    // assert_eq!(ip_cards.len(), 250);
    // assert_eq!(strategy.len(), 750);
    // // assert_eq!(hole_to_string(ip_cards[206]).unwrap(), "KsJs");
    // assert_eq!(strategy[206], 0.0);
    // assert!((strategy[206] + strategy[456] + strategy[706] - 1.0).abs() < 1e-6);

    // // play `Call`
    // game.play(1);

    // // confirm that the current node is a chance node (i.e., river node)
    // assert!(game.is_chance_node());

    // // confirm that "7s" may be dealt
    // let card_7s = Card::from_str("7s").unwrap();
    // assert!(game.possible_cards() & (1 << card_7s.0) != 0);

    // // deal "7s"
    // game.play(card_7s.0 as usize);

    // // back to the root node
    // game.to_root();
}
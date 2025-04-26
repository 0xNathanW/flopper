use crate::{evaluate::load_lookup_table, prelude::*};
use super::{EquityParams, EquityResults, equity_enumerate, equity_monte_carlo};

const LOOKUP_PATH: &str = "./data/lookup_table.bin";
const MC_ITERATIONS: u64 = 100_000;

fn assert_results_within_margin(results: &EquityResults, expected_win_pct: Vec<f64>, margin: f64, is_monte_carlo: bool) {
    let method = if is_monte_carlo { "Monte Carlo" } else { "Enumeration" };
    
    assert_eq!(expected_win_pct.len(), results.wins.len(), "{} failed: Expected length {}, got {}", method, expected_win_pct.len(), results.wins.len());
    
    let total_win_pct: f64 = expected_win_pct.iter().sum();
    let expected_tie_pct = if total_win_pct < 100.0 { 100.0 - total_win_pct } else { 0.0 };
    
    let total = results.total;
    
    for i in 0..expected_win_pct.len() {
        let expected_wins = expected_win_pct[i] * total / 100.0;
        let expected_ties = if expected_tie_pct > 0.0 {
            expected_tie_pct * total / 100.0 / expected_win_pct.len() as f64
        } else {
            0.0
        };
        
        let win_diff = (results.wins[i] - expected_wins).abs();
        let tie_diff = (results.ties[i] - expected_ties).abs();
        
        let effective_margin = if total > 100000.0 { margin * 2.0 } else { margin };
        
        assert!(win_diff <= effective_margin * total / 100.0, 
            "{} failed: Expected wins for player {} to be within {}% of {}, but got {} (diff: {})", 
            method, i, effective_margin, expected_wins, results.wins[i], win_diff
        );
        
        assert!(tie_diff <= effective_margin * total / 100.0, 
            "{} failed: Expected ties for player {} to be within {}% of {}, but got {} (diff: {})", 
            method, i, effective_margin, expected_ties, results.ties[i], tie_diff
        );
    }
}

#[test]
fn test_river_heads_up() {
    let lookup = load_lookup_table(LOOKUP_PATH).unwrap();
    let range_1 = Range::from_str("66+,A8s+,KTs+,QTs+,JTs,ATo+,KJo+").unwrap();
    let range_2 = Range::from_str("22-99,A2o-A8o").unwrap();
    let ranges = vec![range_1, range_2];
    let board = Board::from_str("Qh 4h 8c Qc 6s").unwrap();

    let params_enum = EquityParams {
        ranges: ranges.clone(),
        board: board.clone(),
        lookup: &lookup,
        reporter: None,
    };
    let results_enum = equity_enumerate(params_enum).unwrap();
    assert_results_within_margin(&results_enum, vec![56.0, 44.0], 1.0, false);

    let params_mc = EquityParams {
        ranges: ranges.clone(),
        board,
        lookup: &lookup,
        reporter: None,
    };
    let results_mc = equity_monte_carlo(params_mc, Some(MC_ITERATIONS)).unwrap();
    assert_results_within_margin(&results_mc, vec![56.0, 44.0], 5.0, true);
}

#[test]
fn test_river_multi() {
    let lookup = load_lookup_table(LOOKUP_PATH).unwrap();
    let range_1 = Range::from_str("A7s-A8s,K9s,JTs,ATo,KTo-KJo,QJo").unwrap();
    let range_2 = Range::from_str("66+,A8s+,KTs+,QTs+,JTs,ATo+,KJo+").unwrap();
    let range_3 = Range::from_str("22-TT,KTs+,QTs+,J9s+,T8s+,98s,87s,KJo+,QTo+,JTo,T9o,98o").unwrap();

    let ranges = vec![range_1, range_2, range_3];
    let board = Board::from_str("6h 8s 4s 4d Qc").unwrap();
    
    let params_enum = EquityParams {
        ranges: ranges.clone(),
        board: board.clone(),
        lookup: &lookup,
        reporter: None,
    };
    let results_enum = equity_enumerate(params_enum).unwrap();
    assert_results_within_margin(&results_enum, vec![13.0, 50.0, 37.0], 1.0, false);

    let params_mc = EquityParams {
        ranges: ranges.clone(),
        board,
        lookup: &lookup,
        reporter: None,
    };
    let results_mc = equity_monte_carlo(params_mc, Some(MC_ITERATIONS)).unwrap();
    assert_results_within_margin(&results_mc, vec![13.0, 50.0, 37.0], 5.0, true);
}

#[test]
fn test_turn_multi() {
    let range_1 = Range::from_str("TT+, AKs").unwrap();
    let range_2 = Range::from_str("99+, AKs").unwrap();
    let range_3 = Range::from_str("TT+").unwrap();

    let ranges = vec![range_1, range_2, range_3];
    let board = Board::from_str("Td 6d Qc 8s").unwrap();
    
    let lookup = load_lookup_table(LOOKUP_PATH).unwrap();
    
    let params_enum = EquityParams {
        ranges: ranges.clone(),
        board: board.clone(),
        lookup: &lookup,
        reporter: None,
    };
    let results_enum = equity_enumerate(params_enum).unwrap();
    assert_results_within_margin(&results_enum, vec![32.0, 26.0, 37.0], 1.0, false);

    let params_mc = EquityParams {
        ranges: ranges.clone(),
        board,
        lookup: &lookup,
        reporter: None,
    };
    let results_mc = equity_monte_carlo(params_mc, Some(MC_ITERATIONS)).unwrap();
    assert_results_within_margin(&results_mc, vec![32.0, 26.0, 37.0], 5.0, true);
}

#[test]
fn test_flop_heads_up() {
    let range_1 = Range::from_str("88+,ATs+,KTs+,QJs,AJo+,KQo").unwrap();
    let range_2 = Range::from_str("55,K5s,Q7s,98s,A7o,Q9o,J9o").unwrap();
    let ranges = vec![range_1, range_2];

    let board = Board::from_str("Qh 4h 8c").unwrap();
    let lookup = load_lookup_table(LOOKUP_PATH).unwrap();
    
    let params_enum = EquityParams {
        ranges: ranges.clone(),
        board: board.clone(),
        lookup: &lookup,
        reporter: None,
    };
    let results_enum = equity_enumerate(params_enum).unwrap();
    assert_results_within_margin(&results_enum, vec![66.0, 32.0], 1.0, false);

    let params_mc = EquityParams {
        ranges: ranges.clone(),
        board,
        lookup: &lookup,
        reporter: None,
    };
    let results_mc = equity_monte_carlo(params_mc, Some(MC_ITERATIONS * 2)).unwrap();
    assert_results_within_margin(&results_mc, vec![66.0, 32.0], 5.0, true);
}

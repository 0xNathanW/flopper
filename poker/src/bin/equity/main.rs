use anyhow::{Result, Context};
use clap::Parser;
use prettytable::{Table, Row, Cell};
use rayon::prelude::*;
use poker::{
    board::Board, equity::EquityResults, evaluate::*, range::Range, deck::Deck, card::Card, hand::Hand, remove_dead
};

#[derive(Debug, Parser)]
#[command(author, version)]
#[command(about="Range vs Range equity calculator")]
struct Args {

    #[arg(help = "String represention of ranges to compare. Eg. '22-77' 'A2s+, KQs'")]
    ranges: Vec<String>,

    #[arg(short, long, help = "Board cards (0-5). Eg. '8d Tc 2h', empty for no board")]
    board: Option<String>,

    #[arg(short, long, help = "Path to lookup table")]
    lookup_path: String,
}

fn main() -> Result<()> {

    let args = Args::parse();
    let lookup_path = args.lookup_path;
    let lookup = load_lookup_table(&lookup_path)?;

    let mut ranges = Vec::new();
    if args.ranges.len() < 2 || args.ranges.len() > 8 {
        return Err(anyhow::anyhow!("Number of ranges must be between 2 and 8"));
    }
    for (i, r) in args.ranges.iter().enumerate() {
        let range = Range::from_str(r).context("Failed to parse range")?;
        ranges.push(range);
    }

    let board = if let Some(b) = args.board {
        Board::from_str(&b).context("Failed to parse board")?
    } else {
        Board::default()
    };

    let results = equity_enumerate(ranges, board, &lookup).context("Failed to calculate equity")?;
    print_output(args.ranges, results);

    Ok(())
}

fn print_output(range_str: Vec<String>, results: EquityResults) {

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Range"),
        Cell::new("Equity"),
        Cell::new("Win %"),
        Cell::new("Tie %"),
    ]));

    let equities = results.equities();
    let win_pct = results.wins.iter().map(|w| *w as f64 / results.total as f64 * 100.0).collect::<Vec<f64>>();
    let tie_pct = results.ties.iter().map(|t| *t as f64 / results.total as f64 * 50.0).collect::<Vec<f64>>();

    for i in 0..range_str.len() {
        table.add_row(Row::new(vec![
            Cell::new(range_str[i].as_str()),
            Cell::new(format!("{:.2}%", equities[i]).as_str()),
            Cell::new(format!("{:.2}%", win_pct[i]).as_str()),
            Cell::new(format!("{:.2}%", tie_pct[i]).as_str()),
        ]));
    }

    table.printstd();
}

fn equity_enumerate(ranges: Vec<Range>, board: Board, lookup: &[i32]) -> Result<EquityResults> {

    let board_cards = board.as_vec();
    let (ranges, deck) = remove_dead(ranges, &board_cards)?;

    if board.is_river_dealt() {
        enumerate_river(ranges, &board_cards, lookup)

    } else if board.is_turn_dealt() {
        enumerate_turn(ranges, &board_cards, deck, lookup)
    
    } else if board.is_flop_dealt() {
        enumerate_flop(ranges, &board_cards, deck, lookup)
    
    } else {
        enumerate_preflop(ranges, deck, lookup)
    }
}

fn enumerate_preflop(ranges: Vec<Vec<(Hand, f32)>>, deck: Deck, lookup: &[i32]) -> Result<EquityResults> {

    let results = (0..deck.len()).into_par_iter().map(|a| {

        let mut cards = [Card::default(); 7];
        let mut results = EquityResults::new(ranges.len());

        for b in (a + 1)..deck.len() {
            for c in (b + 1)..deck.len() {
                for d in (c + 1)..deck.len() {
                    for e in (d + 1)..deck.len() {

                        cards[2] = deck[a];
                        cards[3] = deck[b];
                        cards[4] = deck[c];
                        cards[5] = deck[d];
                        cards[6] = deck[e];

                        enumerate_board(&ranges, &mut results, &mut cards, &lookup);
                    }
                }
            }
        }

        results
    }).collect::<Vec<EquityResults>>();

    let mut total = EquityResults::new(ranges.len());
    for result in results {
        total.wins.iter_mut().zip(result.wins.iter()).for_each(|(a, b)| *a += b);
        total.ties.iter_mut().zip(result.ties.iter()).for_each(|(a, b)| *a += b);
        total.total += result.total;
    }

    Ok(total)
}

fn enumerate_flop(ranges: Vec<Vec<(Hand, f32)>>, board: &[Card], deck: Deck, lookup: &[i32]) -> Result<EquityResults> {

    let results = (0..deck.len()).into_par_iter().map(|a| {

        let mut cards = [Card::default(); 7];
        cards[2..5].copy_from_slice(board);
        let mut results = EquityResults::new(ranges.len());

        for b in (a + 1)..deck.len() {

            cards[5] = deck[a];
            cards[6] = deck[b];

            enumerate_board(&ranges, &mut results, &mut cards, &lookup);
        }

        results
    }).collect::<Vec<EquityResults>>();

    let mut total = EquityResults::new(ranges.len());
    for result in results {
        total.wins.iter_mut().zip(result.wins.iter()).for_each(|(a, b)| *a += b);
        total.ties.iter_mut().zip(result.ties.iter()).for_each(|(a, b)| *a += b);
        total.total += result.total;
    }

    Ok(total)
}

fn enumerate_turn(ranges: Vec<Vec<(Hand, f32)>>, board: &[Card], deck: Deck, lookup: &[i32]) -> Result<EquityResults> {

    let results = (0..deck.len()).into_par_iter().map(|a| {
        
        let mut cards = [Card::default(); 7];
        let mut results = EquityResults::new(ranges.len());
        
        cards[2..6].copy_from_slice(board);
        cards[6] = deck[a];
        
        enumerate_board(&ranges, &mut results, &mut cards, &lookup);
        results
    }).collect::<Vec<EquityResults>>();

    let mut total = EquityResults::new(ranges.len());
    for result in results {
        total.wins.iter_mut().zip(result.wins.iter()).for_each(|(a, b)| *a += b);
        total.ties.iter_mut().zip(result.ties.iter()).for_each(|(a, b)| *a += b);
        total.total += result.total;
    }
    Ok(total)
}

fn enumerate_river(ranges: Vec<Vec<(Hand, f32)>>, board: &[Card], lookup: &[i32]) -> Result<EquityResults> {

    let mut results = EquityResults::new(ranges.len());
    let mut cards = [Card::default(); 7];
    cards[2] = board[0];
    cards[3] = board[1];
    cards[4] = board[2];
    cards[5] = board[3];
    cards[6] = board[4];

    enumerate_board(&ranges, &mut results, &mut cards, &lookup);

    Ok(results)
}

fn enumerate_hands(
    ranges: &Vec<Vec<(Hand, f32)>>,
    range_idx: usize,
    used_cards: &mut u64,
    hands: &mut Vec<Hand>,
    board: &mut [Card; 7],
    lookup_table: &[i32],
    results: &mut EquityResults,
) {

    // Base case, one hand assigned to each player.
    if range_idx == ranges.len() {

        let mut best_idxs = [0; 8];
        let mut best_idxs_count = 0;
        let mut best_rank = 0;
        for (i, &hand) in hands.iter().enumerate() {

            board[0] = hand.0;
            board[1] = hand.1;
            
            let rank = eval_7_2p2(board, lookup_table);
            if rank > best_rank {
                best_idxs[0] = i;
                best_idxs_count = 1;
                best_rank = rank;
            } else if rank == best_rank {
                best_idxs[best_idxs_count] = i;
                best_idxs_count += 1;
            }
        }

        // If there is a single best hand, increment the win count for that hand.
        if best_idxs_count == 1 {
            results.wins[best_idxs[0]] += 1;
        } else {
            // If there are multiple best hands, increment the tie count for each.
            for idx in 0..best_idxs_count {
                results.ties[best_idxs[idx]] += 1;
            }
        }

        results.total += 1;
        return;
    }

    for (hand, _weight) in &ranges[range_idx] {

        let hand_mask = 1 << hand.0.0 | 1 << hand.1.0;
        if *used_cards & hand_mask != 0 {
            continue;
        }

        *used_cards |= hand_mask;

        hands.push(*hand);
        enumerate_hands(ranges, range_idx + 1, used_cards, hands, board, lookup_table, results);
        hands.pop();

        *used_cards &= !hand_mask;
    }
}

fn enumerate_board(
    ranges: &Vec<Vec<(Hand, f32)>>,
    results: &mut EquityResults,
    board: &mut [Card; 7],
    lookup_table: &[i32],
) {
    let mut hands = Vec::with_capacity(ranges.len());
    let mut used_cards = 0_u64;
    for card in board[2..].iter() {
        used_cards |= 1 << card.0;
    }

    enumerate_hands(ranges, 0, &mut used_cards, &mut hands, board, lookup_table, results);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const LOOKUP_PATH: &str = "./data/lookup_table.bin";

    pub fn assert_results_eq(results: &EquityResults, equities: Vec<u32>) {
        for (i, pct) in equities.iter().enumerate() {
            assert_eq!((results.equities()[i]).round() as u32, *pct);
        }
    }

    #[test]
    fn test_enumerate_river_heads_up() {

        let lookup = load_lookup_table(LOOKUP_PATH).unwrap();
        let range_1 = Range::from_str("66+,A8s+,KTs+,QTs+,JTs,ATo+,KJo+").unwrap();
        let range_2 = Range::from_str("22-99,A2o-A8o").unwrap();
        let ranges = vec![range_1, range_2];
        let board = Board::from_str("Qh 4h 8c Qc 6s").unwrap();

        let results = equity_enumerate(ranges, board, &lookup).unwrap();
        assert_results_eq(&results, vec![56, 44]);
    }

    #[test]
    fn test_enumerate_river_multi() {
        
        let lookup = load_lookup_table(LOOKUP_PATH).unwrap();
        let range_1 = Range::from_str("A7s-A8s,K9s,JTs,ATo,KTo-KJo,QJo").unwrap();
        let range_2 = Range::from_str("66+,A8s+,KTs+,QTs+,JTs,ATo+,KJo+").unwrap();
        let range_3 = Range::from_str("22-TT,KTs+,QTs+,J9s+,T8s+,98s,87s,KJo+,QTo+,JTo,T9o,98o").unwrap();

        let ranges = vec![range_1, range_2, range_3];
        let board = Board::from_str("6h 8s 4s 4d Qc").unwrap();
        
        let results = equity_enumerate(ranges, board, &lookup).unwrap();
        assert_results_eq(&results, vec![13, 50, 37]);
    }


    #[test]
    fn test_enumerate_turn_multi() {

        let range_1 = Range::from_str("22-TT,KTs+,QTs+,J9s+,T8s+,98s,87s,KJo+,QTo+,JTo,T9o,98o").unwrap();
        let range_2 = Range::from_str("22-TT,K9s+,Q9s+,J8s+,T8s+,97s+,87s,76s,65s,KJo+,QTo+,JTo,T9o,98o").unwrap();
        let range_3 = Range::from_str("22+,A2s+,K2s+,Q2s+,J2s+,T5s+,96s+,87s,76s,A2o+,K3o+,Q6o+,J7o+,T8o+,97o+,87o").unwrap();

        let ranges = vec![range_1, range_2, range_3];
        let board = Board::from_str("Td 6d Qc 8s").unwrap();
        
        let lookup = load_lookup_table(LOOKUP_PATH).unwrap();
        let results = equity_enumerate(ranges, board, &lookup).unwrap();
        assert_results_eq(&results, vec![37, 37, 26]);
    }

    #[test]
    fn test_enumerate_flop_heads_up() {
        let range_1 = Range::from_str("88+,ATs+,KTs+,QJs,AJo+,KQo").unwrap();
        let range_2 = Range::from_str("55,K5s,Q7s,98s,A7o,Q9o,J9o").unwrap();
        let ranges = vec![range_1, range_2];

        let board = Board::from_str("Qh 4h 8c").unwrap();
        let lookup = load_lookup_table(LOOKUP_PATH).unwrap();
        let results = equity_enumerate(ranges, board, &lookup).unwrap();
        assert_results_eq(&results, vec![67, 33]);
    }

    #[test]
    fn test_enumerate_preflop_heads_up() {

        let range_1 = Range::from_str("88+,ATs+,KTs+,QJs,AJo+,KQo").unwrap();
        let range_2 = Range::from_str("55,K5s,Q7s,98s,A7o,Q9o,J9o").unwrap();
        let ranges = vec![range_1, range_2];

        let lookup = load_lookup_table(LOOKUP_PATH).unwrap();
        let results = equity_enumerate(ranges, Board::default(), &lookup).unwrap();
        assert_results_eq(&results, vec![67, 33]);
    }
}
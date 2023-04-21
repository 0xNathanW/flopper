use clap::Parser;
use anyhow::{Result, Context};
use prettytable::{Table, Row, Cell};
use poker::{
    range::Range, 
    card::Card, 
    equity::{equity_enumerate, EquityResults}, 
    hand::HandCombos
};

#[derive(Debug, Parser)]
#[command(author, version)]
#[command(about="Range vs Range equity calculator")]
struct Args {

    #[arg(help = "String represention of ranges to compare. Eg. '22-77, A2s+, KQs'")]
    ranges: Vec<String>,

    #[arg(short, long, help = "Board cards (0-5). Eg. '8d Tc 2h'")]
    board: Option<String>,
}

fn main() -> Result<()> {

    let args = Args::parse();
    
    let mut ranges = Vec::new();
    for (i, r) in args.ranges.iter().enumerate() {
        let range = Range::from_str(&r).with_context(|| format!("Failed to parse range number {}", i))?;
        ranges.push(range.combos());
    }

    let board = if let Some(b) = args.board {
        Card::vec_from_str(&b).context("Failed to parse board")?
    } else {
        Vec::new()
    };

    let results = equity_enumerate(ranges, board).context("Failed to calculate equity")?;
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
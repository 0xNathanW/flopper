use anyhow::{Result, Context};
use clap::Parser;
use prettytable::{Table, Row, Cell};
use poker::{board::Board, equity::EquityResults, evaluate::*, range::Range};

mod enumerate;
use enumerate::equity_enumerate;

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
    for r in args.ranges.iter() {
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
    let win_pct = results.wins.iter().map(|w| *w / results.total * 100.0).collect::<Vec<f64>>();
    let tie_pct = results.ties.iter().map(|t| *t / results.total * 100.0).collect::<Vec<f64>>();

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
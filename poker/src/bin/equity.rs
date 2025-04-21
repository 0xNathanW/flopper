use anyhow::{Result, Context};
use clap::Parser;
use poker::{prelude::*, equity::{equity_enumerate, equity_monte_carlo, EquityParams}};

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
    
    #[arg(short, long, help = "Use Monte Carlo simulation instead of enumeration")]
    monte_carlo: bool,
    
    #[arg(short, long, help = "Number of iterations for Monte Carlo simulation (default: run until SIGINT)")]
    iterations: Option<usize>,
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

    let params = EquityParams {
        ranges,
        board,
        lookup: &lookup,
    };

    let results = if args.monte_carlo {
        println!("Running Monte Carlo simulation{}...", 
                 if let Some(iters) = args.iterations { 
                     format!(" for {} iterations", iters) 
                 } else { 
                     " (Ctrl+C to stop)".to_string() 
                 });
        equity_monte_carlo(params, args.iterations).context("Failed to calculate equity")?
    } else {
        equity_enumerate(params).context("Failed to calculate equity")?
    };
    
    results.print(&args.ranges);

    Ok(())
}

use anyhow::{Result, Context};
use clap::Parser;
use indicatif::{HumanCount, ProgressBar as IndicatifProgressBar, ProgressStyle};
use poker::{equity::{equity_enumerate, equity_monte_carlo, EquityParams, ProgressReporter}, prelude::*};

#[derive(Debug, Parser)]
#[command(author, version)]
#[command(about="Range vs Range equity calculator")]
struct Args {

    #[arg(help = "String represention of ranges to compare. Eg. '22-77' 'A2s+, KQs'")]
    ranges: Vec<String>,

    #[arg(short, long, help = "Board cards (0-5). Eg. '8d Tc 2h', empty for no board")]
    board: Option<String>,

    #[arg(short, long, help = "Path to lookup table")]
    lookup: String,
    
    #[arg(short, long, help = "Use Monte Carlo simulation instead of enumeration")]
    monte_carlo: bool,
    
    #[arg(short, long, help = "Number of iterations for Monte Carlo simulation (default: run until SIGINT)")]
    iterations: Option<u64>,
}

fn main() -> Result<()> {

    let args = Args::parse();
    let lookup_path = args.lookup;
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
    
    let num_runouts = board.num_runouts();
    let progress_bar = {
        if args.monte_carlo {
            if let Some(iterations) = args.iterations {
                println!("Monte Carlo simulation, iterating {} games, sampling 1 matchup per iteration", HumanCount(iterations));
            } else{
                println!("Monte Carlo simulation, iterating until SIGINT, sampling 1 matchup per iteration");
            }
            Box::new(ProgressBar::new(args.iterations))
        } else {
            println!("Enumerating {} runouts", HumanCount(num_runouts));
            Box::new(ProgressBar::new(Some(num_runouts)))
        }
    };

    let params = EquityParams {
        ranges,
        board,
        lookup: &lookup,
        reporter: Some(progress_bar.as_ref() as &dyn ProgressReporter),
    };

    let results = if args.monte_carlo {
        equity_monte_carlo(params, args.iterations)
    } else {
        equity_enumerate(params)
    }.context("Failed to calculate equity")?;
    
    progress_bar.finish();
    results.print(&args.ranges);
    Ok(())
}

struct ProgressBar {
    bar: IndicatifProgressBar,
    
}

impl ProgressBar {
    fn new(len: Option<u64>) -> Self {
        let bar = match len {
            Some(n) => {
                let bar = IndicatifProgressBar::new(n);
                bar.set_style(ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {human_pos}/{human_len}")
                    .unwrap()
                    .progress_chars("#>-"));
                bar
            },
            None => {
                let bar = IndicatifProgressBar::new_spinner();
                bar.set_style(ProgressStyle::default_spinner()
                    .template("{spinner:.green} [{elapsed_precise}] {human_pos} games")
                    .unwrap());
                bar
            }
        };
        Self { bar }
    }

    fn finish(&self) {
        self.bar.finish();
    }
}

impl ProgressReporter for ProgressBar {
    fn board_complete(&self) {
        self.bar.inc(1);
    }
}

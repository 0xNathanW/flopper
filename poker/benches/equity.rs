use criterion::{black_box, criterion_group, criterion_main, Criterion, SamplingMode};
use poker::{prelude::*, equity::{EquityParams, equity_monte_carlo, equity_enumerate}};
use rand::{rngs::StdRng, SeedableRng};
use std::path::Path;

// TODO: Maybe change to randomised ranges
fn fixed_ranges(num_players: usize) -> Vec<Range> {
    let mut ranges = Vec::with_capacity(num_players);
    
    let range_strings = [
        "AA,KK,QQ,JJ,TT",
        "AKs,AQs,AJs,ATs,KQs"
    ];
    
    for i in 0..num_players {
        let idx = i % range_strings.len();
        ranges.push(Range::from_str(range_strings[idx]).unwrap());
    }
    
    ranges
}

fn generate_random_board(num_cards: usize, rng: &mut StdRng) -> Board {
    if num_cards == 0 {
        return Board::default();
    }
    
    let mut deck = Deck::new();
    deck.shuffle(rng);
    
    let cards = deck.pop_n(num_cards);
    Board::from_vec(cards).unwrap()
}

fn benchmark_monte_carlo(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(42);
    
    let lookup_table = match load_lookup_table(Path::new("data/lookup_table.bin")) {
        Ok(table) => table,
        Err(e) => {
            eprintln!("Failed to load lookup table: {}", e);
            return;
        }
    };
    
    let mut group = c.benchmark_group("monte_carlo");
    group.sample_size(30); 
    group.sampling_mode(SamplingMode::Flat);
    group.measurement_time(std::time::Duration::from_secs(10));
    
    group.bench_function("preflop_2p", |b| {
        b.iter_with_setup(
            || {
                let ranges = fixed_ranges(2);
                let board = generate_random_board(0, &mut rng);
                EquityParams {
                    ranges,
                    board,
                    lookup: &lookup_table,
                }
            },
            |params| black_box(equity_monte_carlo(params, Some(50)).unwrap())
        )
    });
    
    group.bench_function("flop_2p", |b| {
        b.iter_with_setup(
            || {
                let ranges = fixed_ranges(2);
                let board = generate_random_board(3, &mut rng);
                EquityParams {
                    ranges,
                    board,
                    lookup: &lookup_table,
                }
            },
            |params| black_box(equity_monte_carlo(params, Some(50)).unwrap())
        )
    });
    
    group.bench_function("turn_2p", |b| {
        b.iter_with_setup(
            || {
                let ranges = fixed_ranges(2);
                let board = generate_random_board(4, &mut rng);
                EquityParams {
                    ranges,
                    board,
                    lookup: &lookup_table,
                }
            },
            |params| black_box(equity_monte_carlo(params, Some(50)).unwrap())
        )
    });
    
    group.bench_function("river_2p", |b| {
        b.iter_with_setup(
            || {
                let ranges = fixed_ranges(2);
                let board = generate_random_board(5, &mut rng);
                EquityParams {
                    ranges,
                    board,
                    lookup: &lookup_table,
                }
            },
            |params| black_box(equity_monte_carlo(params, Some(50)).unwrap())
        )
    });
    
    group.finish();
}

fn benchmark_enumerate(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(42);
    
    let lookup_table = match load_lookup_table(Path::new("data/lookup_table.bin")) {
        Ok(table) => table,
        Err(e) => {
            eprintln!("Failed to load lookup table: {}", e);
            return;
        }
    };
    
    let mut group = c.benchmark_group("enumerate");
    group.sample_size(20);
    group.sampling_mode(SamplingMode::Flat);
    group.measurement_time(std::time::Duration::from_secs(15));
    
    group.bench_function("turn_2p", |b| {
        b.iter_with_setup(
            || {
                let ranges = fixed_ranges(2);
                let board = generate_random_board(4, &mut rng);
                EquityParams {
                    ranges,
                    board,
                    lookup: &lookup_table,
                }
            },
            |params| black_box(equity_enumerate(params).unwrap())
        )
    });
    
    group.bench_function("river_2p", |b| {
        b.iter_with_setup(
            || {
                let ranges = fixed_ranges(2);
                let board = generate_random_board(5, &mut rng);
                EquityParams {
                    ranges,
                    board,
                    lookup: &lookup_table,
                }
            },
            |params| black_box(equity_enumerate(params).unwrap())
        )
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_monte_carlo,
    benchmark_enumerate
);
criterion_main!(benches);

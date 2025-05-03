use criterion::{black_box, criterion_group, criterion_main, Criterion};
use poker::{prelude::*, evaluate::{rank_hand_bits, rank_hand_senzee, rank_hand_2p2, load_lookup_table}};
use std::path::Path;

fn generate_random_hand(size: usize) -> Vec<Card> {
    let mut deck = Deck::new();
    deck.shuffle();
    deck.pop_n(size)
}

fn benchmark_naive(c: &mut Criterion) {
    let mut group = c.benchmark_group("bits");
    
    group.bench_function("5-card", |b| {
        b.iter_with_setup(
            || generate_random_hand(5),
            |hand| black_box(rank_hand_bits(&hand).unwrap())
        )
    });
    
    group.bench_function("7-card", |b| {
        b.iter_with_setup(
            || generate_random_hand(7),
            |hand| black_box(rank_hand_bits(&hand).unwrap())
        )
    });
    
    group.finish();
}

fn benchmark_senzee(c: &mut Criterion) {
    let mut group = c.benchmark_group("senzee");
    
    group.bench_function("5-card", |b| {
        b.iter_with_setup(
            || generate_random_hand(5),
            |hand| black_box(rank_hand_senzee(&hand).unwrap())
        )
    });
    
    group.bench_function("7-card", |b| {
        b.iter_with_setup(
            || generate_random_hand(7),
            |hand| black_box(rank_hand_senzee(&hand).unwrap())
        )
    });
    
    group.finish();
}

fn benchmark_two_plus_two(c: &mut Criterion) {
    
    let lookup_path = std::env::var("LOOKUP_PATH").unwrap_or("data/lookup_table.bin".to_string());
    let lookup_table = match load_lookup_table(Path::new(&lookup_path)) {
        Ok(table) => table,
        Err(e) => {
            eprintln!("Failed to load lookup table: {}", e);
            return;
        }
    };
    
    let mut group = c.benchmark_group("two_plus_two");
    
    group.bench_function("5-card", |b| {
        b.iter_with_setup(
            || generate_random_hand(5),
            |hand| black_box(rank_hand_2p2(&hand, &lookup_table).unwrap())
        )
    });
    
    group.bench_function("7-card", |b| {
        b.iter_with_setup(
            || generate_random_hand(7),
            |hand| black_box(rank_hand_2p2(&hand, &lookup_table).unwrap())
        )
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_naive,
    benchmark_senzee,
    benchmark_two_plus_two
);
criterion_main!(benches);

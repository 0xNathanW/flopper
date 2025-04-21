use criterion::{black_box, criterion_group, criterion_main, Criterion};
use poker::{
    card::{Card, Rank, Suit},
    evaluate::{rank_hand_naive, rank_hand_senzee, rank_hand_2p2, load_lookup_table},
};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use std::path::Path;

fn generate_random_hand(size: usize, rng: &mut StdRng) -> Vec<Card> {
    let mut deck: Vec<Card> = (0..52).map(|i| {
        let rank = (i % 13) as u8;
        let suit = (i / 13) as u8;
        Card::new(Rank::from(rank), Suit::from(suit))
    }).collect();
    deck.shuffle(rng);
    deck.into_iter().take(size).collect()
}

fn benchmark_naive(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(42);
    let mut group = c.benchmark_group("naive");
    
    group.bench_function("5-card", |b| {
        b.iter_with_setup(
            || generate_random_hand(5, &mut rng),
            |hand| black_box(rank_hand_naive(&hand).unwrap())
        )
    });
    
    group.bench_function("7-card", |b| {
        b.iter_with_setup(
            || generate_random_hand(7, &mut rng),
            |hand| black_box(rank_hand_naive(&hand).unwrap())
        )
    });
    
    group.finish();
}

fn benchmark_senzee(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(42);
    let mut group = c.benchmark_group("senzee");
    
    group.bench_function("5-card", |b| {
        b.iter_with_setup(
            || generate_random_hand(5, &mut rng),
            |hand| black_box(rank_hand_senzee(&hand).unwrap())
        )
    });
    
    group.bench_function("7-card", |b| {
        b.iter_with_setup(
            || generate_random_hand(7, &mut rng),
            |hand| black_box(rank_hand_senzee(&hand).unwrap())
        )
    });
    
    group.finish();
}

fn benchmark_two_plus_two(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(42);
    
    let table_path = Path::new("data/lookup_table.bin");
    let lookup_table = match load_lookup_table(table_path) {
        Ok(table) => table,
        Err(e) => {
            eprintln!("Failed to load lookup table: {}", e);
            return;
        }
    };
    
    let mut group = c.benchmark_group("two_plus_two");
    
    group.bench_function("5-card", |b| {
        b.iter_with_setup(
            || generate_random_hand(5, &mut rng),
            |hand| black_box(rank_hand_2p2(&hand, &lookup_table).unwrap())
        )
    });
    
    group.bench_function("7-card", |b| {
        b.iter_with_setup(
            || generate_random_hand(7, &mut rng),
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

use std::collections::HashSet;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use poker::card::Card;
use poker::evaluate::{
    rank_cards_two_plus_two,
    rank_cards_naive,
    rank_cards_senzee,
};

// Generate 100,000 random hands, making s.
fn generate_hands() -> Vec<[Card; 7]> {
    println!("Generating hands...");
    let mut rng = rand::thread_rng();
    let mut hands = Vec::with_capacity(100_000);
    for _ in 0..100_000 {
        let mut hand = [Card::default(); 7];
        let mut seen = HashSet::with_capacity(7);
        for i in 0..7 {
            let mut card = Card::random(&mut rng);
            while seen.contains(&card) {
                card = Card::random(&mut rng);
            }
            hand[i] = card;
            seen.insert(card);
        }
        hands.push(hand);
    }
    hands
}
// Current - calculates approx. 2,640,194 hands per second.
pub fn bench_two_plus_two_100000(c: &mut Criterion) {
    let hands = generate_hands();
    let lookup_table = poker::evaluate::load_lookup_table().unwrap();
    c.bench_function("two_plus_two", |b| b.iter(|| {
        for hand in &hands {
            black_box(rank_cards_two_plus_two(hand, &lookup_table));
        }
    }));
}

pub fn bench_senzee_100000(c: &mut Criterion) {
    let hands = generate_hands();
    c.bench_function("senzee", |b| b.iter(|| {
        for hand in &hands {
            black_box(rank_cards_senzee(hand));
        }
    }));
}

pub fn bench_naive_100000(c: &mut Criterion) {
    let hands = generate_hands();
    c.bench_function("naive", |b| b.iter(|| {
        for hand in &hands {
            black_box(rank_cards_naive(hand));
        }
    }));
}

criterion_group!(benches, bench_two_plus_two_100000, bench_senzee_100000, bench_naive_100000);
criterion_main!(benches);


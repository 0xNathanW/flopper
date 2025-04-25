use std::time;
use clap::{Parser, ValueEnum};
use poker::{evaluate::{rank_hand_2p2, rank_hand_bits, rank_hand_senzee}, prelude::*};

const COMBO_COUNT: usize = 133_784_560;

#[derive(Parser)]
#[command(name = "Poker Hand Evaluator Profiler")]
#[command(author = "Poker Library")]
#[command(version = "1.0")]
#[command(about = "Profiles different poker hand evaluation methods", long_about = None)]
struct Args {

    #[arg(long, value_enum, default_value_t = EnumerationMethod::Random)]
    order: EnumerationMethod,
    
    #[arg(long, value_enum, default_value_t = EvaluationMethod::TwoPlusTwo)]
    eval: EvaluationMethod,

}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
enum EnumerationMethod {
    Random,
    Sequential,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
enum EvaluationMethod {
    Bits,
    Senzee,
    TwoPlusTwo,
}

fn main() {
    let args = Args::parse();
    
    let cards = Deck::new();
    let mut hand = [Card::default(); 7];    
    let lookup = if args.eval == EvaluationMethod::TwoPlusTwo {
        Some(load_lookup_table("data/lookup_table.bin").unwrap())
    } else {
        None
    };

    let start = time::Instant::now();

    // There are 133,784,560 7 card combinations in a deck of 52 cards.
    match args.order {
        EnumerationMethod::Random => {
            for _ in 0..COMBO_COUNT {
                let idxs = random_idxs();
                hand[0] = cards[idxs[0]];
                hand[1] = cards[idxs[1]];
                hand[2] = cards[idxs[2]];
                hand[3] = cards[idxs[3]];
                hand[4] = cards[idxs[4]];
                hand[5] = cards[idxs[5]];
                hand[6] = cards[idxs[6]];
                
                evaluate_hand(&hand, args.eval, &lookup);
            }
        },
        EnumerationMethod::Sequential => {
            for a in 0..52 {
                for b in (a + 1)..52 {
                    for c in (b + 1)..52 {
                        for d in (c + 1)..52 {
                            for e in (d + 1)..52 {
                                for f in (e + 1)..52 {
                                    for g in (f + 1)..52 {
                                        
                                        hand[0] = cards[a];
                                        hand[1] = cards[b];
                                        hand[2] = cards[c];
                                        hand[3] = cards[d];
                                        hand[4] = cards[e];
                                        hand[5] = cards[f];
                                        hand[6] = cards[g];
                                        
                                        evaluate_hand(&hand, args.eval, &lookup);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
}

#[inline(always)]
fn evaluate_hand(hand: &[Card; 7], method: EvaluationMethod, lookup: &Option<Vec<i32>>) {
    match method {
        EvaluationMethod::Bits => {
            let _ = rank_hand_bits(hand).unwrap();
        },
        EvaluationMethod::Senzee => {
            let _ = rank_hand_senzee(hand).unwrap();
        },
        EvaluationMethod::TwoPlusTwo => {
            if let Some(lookup_table) = lookup {
                let _ = rank_hand_2p2(hand, lookup_table).unwrap();
            }
        },
    }
}

#[inline(always)]
fn random_idxs() -> [usize; 7] {
    let mut indices = [0; 7];
    let mut i = 0;
    
    while i < 7 {
        let num = fastrand::usize(0..52);
        // Check if number is already used
        if !indices[..i].contains(&num) {
            indices[i] = num;
            i += 1;
        }
    }
    
    indices
}

use std::{collections::{BTreeMap, VecDeque}, fs::File, io::Write, path::PathBuf};
use anyhow::Result;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use poker::evaluate::{eval_5, eval_6, eval_7};

const TABLE_SIZE: usize = 32_487_834;
const PRIMES: [i32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, help = "Path to save the lookup table")]
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let table = generate_lookup_table();
    println!("Saving lookup table to {}...", args.path.display());
    save_lookup_table(table, &args.path)
}

fn save_lookup_table(table: Vec<i32>, path: &PathBuf) -> Result<()> {
    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    let mut file = File::create(path)?;
    let buffer: &[u8] = unsafe {
        std::slice::from_raw_parts(
            table.as_ptr() as *const u8,
            TABLE_SIZE * 4,
        )
    };
    file.write_all(buffer)?;
    Ok(())
}

fn generate_lookup_table() -> Vec<i32> {
    let mut lookup_table = vec![0; TABLE_SIZE];

    let mut sub_hands: BTreeMap<u64, i64> = BTreeMap::new();
    let mut sub_hand_queue: VecDeque<u64> = VecDeque::new();

    sub_hand_queue.push_back(0);
    sub_hands.insert(0, 0);

    // Setup the progress bar for enumerating sub hands
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner} [{elapsed_precise}] {msg}")
            .unwrap()
    );
    progress_bar.set_message("Enumerating sub hands...");

    // Enumerate all possible sub hands.
    let mut count = 0;
    while !sub_hand_queue.is_empty() {
        count += 1;
        if count % 1000 == 0 {
            progress_bar.set_message(format!("Enumerating sub hands... {} processed", count));
        }
        
        let sub_hand = sub_hand_queue.pop_front().unwrap();
        for card in 1..53 {
            
            let (num_cards, id) = get_id(sub_hand as i64, card);
            let id = match id {
                Some(id) => id,
                None => continue,
            };
            
            let returned = sub_hands.insert(id as u64, 0);
            if returned.is_none() && num_cards < 6 {
                sub_hand_queue.push_back(id as u64);
            }
        }
    }

    progress_bar.finish_with_message("Sub hands enumeration completed");

    for (idx, (_, val)) in sub_hands.iter_mut().enumerate() {
        *val = idx as i64;
    }

    // Setup the progress bar for the main table generation
    let pb = ProgressBar::new(sub_hands.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40}] {pos}/{len} ({percent}%)")
            .unwrap()
            .progress_chars("=> ")
    );

    for (sub_hand, sub_hand_pos) in sub_hands.iter() {
        let mut num_cards = 0; 

        for c in 1..53 {

            let max_hr = sub_hand_pos * 53 + c + 53;
            let (n, id) = get_id(*sub_hand as i64, c as i32);
            let id = id.unwrap_or(0);
            
            num_cards = n;
            if num_cards == 7 {
                lookup_table[max_hr as usize] = do_eval(id);
                continue;
            }
            if id == 0 {
                continue;
            }

            let position = sub_hands.get(&(id as u64)).expect("id not found");
            lookup_table[max_hr as usize] = (position * 53 + 53) as i32;
        }
    
        if num_cards == 6 || num_cards == 7 {
            lookup_table[(sub_hand_pos * 53 + 53) as usize] = do_eval(*sub_hand as i64);
        }

        pb.inc(1);
    }

    pb.finish();
    lookup_table
}

// Returns a 64-bit hand ID, for up to 8 cards, one per bye.
fn get_id(id_in: i64, card: i32) -> (i32, Option<i64>) {

    let mut suit_count = [0_i32; 5];
    let mut rank_count = [0_i32; 14];
    let mut work_cards = [0_i32; 8];
    let mut num_cards = 0;
    let mut get_out = 0;
    let new_card = card - 1;

    for n in 0..6 {
        work_cards[n + 1] = ((id_in >> (n *8)) & 0xFF) as i32;
    }

    
    work_cards[0] = (((new_card >> 2) + 1) << 4) + (new_card & 0x3) + 1;
    
    while work_cards[num_cards] != 0 {
        
        suit_count[(work_cards[num_cards] as usize) & 0xF] += 1;
        rank_count[(work_cards[num_cards] as usize) >> 4] += 1;

        if num_cards != 0 && work_cards[0] == work_cards[num_cards] {
            get_out = 1;
        }

        num_cards += 1;
    }

    if get_out != 0 {
        return (num_cards as i32, None);
    }

    let need_suited = num_cards as i32 - 2;
    if num_cards > 4 {
        for rank in 1..14 {
            if rank_count[rank] > 4 {
                return (num_cards as i32, None);
            }
        }
    }

    if need_suited > 1 {
        for n in 0..num_cards {
            if suit_count[(work_cards[n] as usize) & 0xF] < need_suited {
                work_cards[n] &= 0xF0;
            }
        }
    }

    macro_rules! swap {
        ($i: expr, $j: expr) => {
            if work_cards[$i] < work_cards[$j] {
                work_cards[$i] ^= work_cards[$j];
                work_cards[$j] ^= work_cards[$i];
                work_cards[$i] ^= work_cards[$j];
            }
        };
    }

    swap!(0, 4);
    swap!(1, 5);
    swap!(2, 6);
    swap!(0, 2);
    swap!(1, 3);
    swap!(4, 6);
    swap!(2, 4);
    swap!(3, 5);
    swap!(0, 1);
    swap!(2, 3);
    swap!(4, 5);
    swap!(1, 4);
    swap!(3, 6);
    swap!(1, 2);
    swap!(3, 4);
    swap!(5, 6);

    (
        num_cards as i32,
        Some(work_cards[0] as i64 + 
            ((work_cards[1] as i64) << 8) +
            ((work_cards[2] as i64) << 16) +
            ((work_cards[3] as i64) << 24) +
            ((work_cards[4] as i64) << 32) +
            ((work_cards[5] as i64) << 40) +
            ((work_cards[6] as i64) << 48)
        )
    )
}

pub fn do_eval(id: i64) -> i32 {

    let mut main_suit = 20;
    let mut suit_iter = 1;

    let mut work_cards = [0_i32; 8];
    let mut hold_cards = [0_i32; 8];
    let mut num_eval_cards = 0;

    if id == 0 { return 0; } // Bad id.

    for c in 0..7 {

        hold_cards[c] = ((id >> (c * 8)) & 0xFF) as i32;
        if hold_cards[c] == 0 {
            break;
        }
        num_eval_cards += 1;

        let suit = hold_cards[c] & 0xF;
        if suit != 0 {
            main_suit = suit;
        }
    }

    for c in 0..num_eval_cards {

        let work_card = hold_cards[c];
        let rank = (work_card >> 4) - 1;
        let mut suit = work_card & 0xF;

        if suit == 0 {
            suit = suit_iter;
            suit_iter += 1;

            if suit_iter == 5 {
                suit_iter = 1;
            }

            if suit == main_suit {
                suit = suit_iter;
                suit_iter += 1;

                if suit_iter == 5 {
                    suit_iter = 1;
                }
            }
        }

        //   +--------+--------+--------+--------+
        //   |xxxbbbbb|bbbbbbbb|cdhsrrrr|xxpppppp|
        //   +--------+--------+--------+--------+
        //   p = prime number of rank (deuce=2,trey=3,four=5,five=7,...,ace=41)
        //   r = rank of card (deuce=0,trey=1,four=2,five=3,...,ace=12)
        //   cdhs = suit of card
        //   b = bit turned on depending on rank of card
        work_cards[c] = (1 << (16 + rank)) | (1 << (suit + 11)) | (rank << 8) | PRIMES[rank as usize] 
    }

    let mut new_work_cards = [0_u32; 8]; 
    for c in 0..num_eval_cards {
        new_work_cards[c] = work_cards[c] as u32;
    }

    match num_eval_cards {
        5 => eval_5(&new_work_cards[0..5]) as i32,
        6 => eval_6(&new_work_cards[0..6]) as i32,
        7 => eval_7(&new_work_cards[0..7]) as i32,
        _ => unreachable!(),
    }
}

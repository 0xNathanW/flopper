use std::collections::{BTreeMap, VecDeque};

const TABLE_SIZE: usize = 32_487_834;

pub fn generate_lookup_table() -> [i32; TABLE_SIZE] {

    let mut lookup_table = [0_i32; TABLE_SIZE];

    let mut sub_hands: BTreeMap<u64, i64> = BTreeMap::new();
    let mut sub_hand_queue: VecDeque<u64> = VecDeque::new();

    sub_hand_queue.push_back(0);
    sub_hands.insert(0, 0);

    // Enumerate all possible sub hands.
    while !sub_hand_queue.is_empty() {
        
        let sub_hand = sub_hand_queue.pop_front().unwrap();
        for card in 0..53 {
            
            let (num_cards, id) = get_id(sub_hand as i64, card);
            let id = match id {
                Some(id) => id,
                None => continue,
            };
            
            let returned = sub_hands.insert(id as u64, 0);
            if returned.is_none() && num_cards < 7 {
                sub_hand_queue.push_back(id as u64);
            }
        }
    }

    for (idx, (_, val)) in sub_hands.iter_mut().enumerate() {
        *val = idx as i64;
    }

    for (sub_hand, sub_hand_pos) in sub_hands.iter() {
        let mut num_cards = 0; 

        for c in 1..53 {

            let max_hr = sub_hand_pos * 53 + c + 53;
            let (n, id) = get_id(*sub_hand as i64, c as i32);
            let id = id.unwrap_or(0);
            
            num_cards = n;
            if num_cards == 7 {
                // do eval 
                continue;
            }
            if id == 0 {
                continue;
            }

            let position = sub_hands.get(&(id as u64)).expect("id not found");
            lookup_table[max_hr as usize] = (position * 53 + 53) as i32;
        }
    
        if num_cards == 6 || num_cards == 7 {
            // lookup_table[(sub_hand_pos * 53 + 53) as usize] = do_eval
        }
    }

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

        if num_cards != 0 {
            if work_cards[0] == work_cards[num_cards] {
                get_out = 1;
            }
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
            (work_cards[1] as i64) << 8 +
            (work_cards[2] as i64) << 16 +
            (work_cards[3] as i64) << 24 +
            (work_cards[4] as i64) << 32 +
            (work_cards[5] as i64) << 40 +
            (work_cards[6] as i64) << 48
        )
    )
}

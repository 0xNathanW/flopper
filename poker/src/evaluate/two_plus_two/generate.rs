use std::collections::{BTreeMap, VecDeque};
use std::io::{Write, Result, Read};
use std::fs::File;
use super::eval::do_eval;

const TABLE_SIZE: usize = 32_487_834;

pub fn generate_lookup_table() -> Vec<i32> {

    let mut lookup_table = vec![0; TABLE_SIZE];

    let mut sub_hands: BTreeMap<u64, i64> = BTreeMap::new();
    let mut sub_hand_queue: VecDeque<u64> = VecDeque::new();

    sub_hand_queue.push_back(0);
    sub_hands.insert(0, 0);

    // Enumerate all possible sub hands.
    while !sub_hand_queue.is_empty() {
        
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
            lookup_table[(sub_hand_pos * 53 + 53) as usize] = do_eval(*sub_hand as i64)
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

pub fn save_lookup_table(lookup_table: Vec<i32>) -> Result<()> {

        let mut file = File::create("lookup_table.bin")?;
        let buffer: &[u8] = unsafe {
            std::slice::from_raw_parts(
                lookup_table.as_ptr() as *const u8,
                TABLE_SIZE * 4,
            )
        };
    
        file.write_all(buffer)?;
        Ok(())
}

pub fn load_lookup_table() -> Result<Vec<i32>> {
    let mut file = {
        match File::open("lookup_table.bin") {
            Ok(file) => file,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    File::open("poker/lookup_table.bin")?
                },
                _ => return Err(e),
            }
        }
    };
    let mut buffer = vec![0_u8; TABLE_SIZE * 4];
    let n = file.read(&mut buffer)?;
    if n != TABLE_SIZE * 4 {
        return Err(std::io::ErrorKind::UnexpectedEof.into());
    }

    let lookup_table: Vec<i32> = unsafe {
        let ptr = buffer.as_ptr() as *mut i32;
        std::mem::forget(buffer);
        Vec::from_raw_parts(ptr, TABLE_SIZE, TABLE_SIZE)
    };
    Ok(lookup_table)
}

#[cfg(test)]
mod tests {
    use super::*;
    use md5::{Digest, Md5};
    use hex_literal::hex;

    #[test]
    fn test_lookup_hash() {
        let lookup_table = generate_lookup_table();
        // MD5 sum should be 5003cf3e6d5c9b8ee77094e168bfe73f
        let mut hasher = Md5::new();
        let buffer: &[u8] = unsafe {
            std::slice::from_raw_parts(
                lookup_table.as_ptr() as *const u8,
                TABLE_SIZE * 4,
            )
        };
        hasher.update(&buffer);
        let result = hasher.finalize();
        assert_eq!(result[..], hex!("5003cf3e6d5c9b8ee77094e168bfe73f"))
    }

    #[test]
    fn test_lookup_save_load_hash() {
        let lookup_table = match load_lookup_table() {
            Ok(lookup_table) => lookup_table,
            Err(_) => {
                let lookup_table = generate_lookup_table();
                save_lookup_table(lookup_table.clone()).unwrap();
                lookup_table
            }
        };
        // MD5 sum should be 5003cf3e6d5c9b8ee77094e168bfe73f
        let mut hasher = Md5::new();
        let buffer: &[u8] = unsafe {
            std::slice::from_raw_parts(
                lookup_table.as_ptr() as *const u8,
                TABLE_SIZE * 4,
            )
        };
        hasher.update(&buffer);
        let result = hasher.finalize();
        assert_eq!(result[..], hex!("5003cf3e6d5c9b8ee77094e168bfe73f"))
    }
}
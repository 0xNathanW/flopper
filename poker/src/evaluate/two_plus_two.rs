use std::{io::{self, Read}, path::Path};
use crate::card::Card;
use super::HandRank;
use crate::error::{Error, Result};

const TABLE_SIZE: usize = 32_487_834;

pub fn load_lookup_table<P: AsRef<Path>>(path: P) -> Result<Vec<i32>> {
    let mut buffer = vec![0_u8; TABLE_SIZE * 4];
    
    let mut file = std::fs::File::open(&path).map_err(|e| {
        if e.kind() == io::ErrorKind::NotFound {
            Error::LookupTableNotFound(path.as_ref().display().to_string())
        } else {
            Error::LookupTableError(e)
        }
    })?;

    let n = file.read(&mut buffer)?;
    if n != TABLE_SIZE * 4 {
        return Err(Error::LookupTableError(io::Error::new(io::ErrorKind::UnexpectedEof, "wrong table size")));
    }
    
    let lookup_table: Vec<i32> = unsafe {
        let ptr = buffer.as_ptr() as *mut i32;
        std::mem::forget(buffer);
        Vec::from_raw_parts(ptr, TABLE_SIZE, TABLE_SIZE)
    };
    
    Ok(lookup_table)
}

pub fn rank_hand_two_plus_two(hand: &[Card], lookup_table: &[i32]) -> Result<HandRank> {
    assert!(hand.len() >= 5 && hand.len() <= 7);
    let rank = match hand.len() {
        5 => rank_hand_5(hand, lookup_table),
        6 => rank_hand_6(hand, lookup_table),
        7 => rank_hand_7(hand, lookup_table),
        _ => return Err(Error::InvalidHandSize(hand.len())),
    };

    Ok(HandRank::from(rank))
}

#[inline]
pub fn rank_hand_5(hand: &[Card], lookup_table: &[i32]) -> u16 {
    let mut r = lookup_table[53 + hand[0].0 as usize + 1] as usize;
    r = lookup_table[r + hand[1].0 as usize + 1] as usize;
    r = lookup_table[r + hand[2].0 as usize + 1] as usize;
    r = lookup_table[r + hand[3].0 as usize + 1] as usize;
    r = lookup_table[r + hand[4].0 as usize + 1] as usize;
    r = lookup_table[r] as usize;
    r as u16
}

#[inline]
pub fn rank_hand_6(hand: &[Card], lookup_table: &[i32]) -> u16 {
    let mut r = lookup_table[53 + hand[0].0 as usize + 1] as usize;
    r = lookup_table[r + hand[1].0 as usize + 1] as usize;
    r = lookup_table[r + hand[2].0 as usize + 1] as usize;
    r = lookup_table[r + hand[3].0 as usize + 1] as usize;
    r = lookup_table[r + hand[4].0 as usize + 1] as usize;
    r = lookup_table[r + hand[5].0 as usize + 1] as usize;
    r = lookup_table[r] as usize;
    r as u16
}

#[inline]
pub fn rank_hand_7(hand: &[Card], lookup_table: &[i32]) -> u16 {
    let mut r = lookup_table[53 + hand[0].0 as usize + 1] as usize;
    r = lookup_table[r + hand[1].0 as usize + 1] as usize;
    r = lookup_table[r + hand[2].0 as usize + 1] as usize;
    r = lookup_table[r + hand[3].0 as usize + 1] as usize;
    r = lookup_table[r + hand[4].0 as usize + 1] as usize;
    r = lookup_table[r + hand[5].0 as usize + 1] as usize;
    r = lookup_table[r + hand[6].0 as usize + 1] as usize;
    r as u16
}
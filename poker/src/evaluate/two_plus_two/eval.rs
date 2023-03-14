use super::super::senzee::*;

const PRIMES: [i32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

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

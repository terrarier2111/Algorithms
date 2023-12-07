#![feature(int_roundings)]
#![feature(const_mut_refs)]
#![feature(const_maybe_uninit_write)]

mod sorting;
mod encrypt;
mod break_encrypt;
pub mod util;
mod search;
mod sudoku;

use std::collections::HashMap;
use std::mem::transmute;
use std::slice::Iter;
use std::collections::hash_map::Values;
use std::convert::TryInto;

use crate::sudoku::print_constants;

fn main() {
    /*let mut unsorted = [3, 7, 34, 937, 543, 63, 8427, 0, 1, 934, 1, 47427, 245];
    println!("{:?}", unsorted);
    sorting::insertion::sort(&mut unsorted);
    println!("sorted: {:?}", unsorted);
    let sorted = [1, 2, 3, 4, 5, 6, 7, 9];
    println!("found: {:?}", search::binary::search_length_limited_nearest(&sorted, /*3*/8/*100*/, sorted.len()));*/
    /*
    let sorted = sorting::radix::sort(&unsorted);
    println!("{:?}", sorted);
    // let mut input = "test".to_string().to_uppercase();
    // let mut caesar = encrypt::encrypt_caesar(&mut input, 13);
    let caesar = "LFK NDP VDK XQG VLH JWH".to_string();
    println!("broken: {:?}", break_encrypt::caesar::break_caesar_permuts_default(caesar.to_uppercase().as_mut_str())/*.unwrap()*/);
    let poly_result = encrypt::encrypt_poly("DERABGESCHLOSSENEROMAn", "JAMESBOND");
    println!("poly: {}", poly_result);
    println!("decrypted poly: {}", encrypt::decrypt_poly(&*poly_result, "JAMESBOND"));
    let to_query_through: &[usize] = &[2, 5, 8, 30, 60, 80, 98, 347, 348, 9423];
    println!("found position: {}", search::binary::search(to_query_through, 30));*/
    let mut example = EXAMPLE_SUDOKU_BOARD.clone();
    sudoku::solver::solve(&mut example);
    let ex_print = unsafe { transmute::<_, [[u8; 9]; 9]>(example.clone()) };
    for row in ex_print {
        println!("{:?}", row);
    }
    assert_ne!(&example, EXAMPLE_SUDOKU_BOARD);
    println!("tz: {}", (1_usize << 8).trailing_zeros() as usize);
    print_constants();
}

const EXAMPLE_SUDOKU_BOARD: &[u8; 81] = &[
    0, 2, 0, 0, 0, 0, 0, 0, 0, // row 1
    0, 0, 0, 6, 0, 0, 0, 0, 3, // row 2
    0, 7, 4, 0, 8, 0, 0, 0, 0, // row 3
    0, 0, 0, 0, 0, 3, 0, 0, 2, // row 4
    0, 8, 0, 0, 4, 0, 0, 1, 0, // row 5
    6, 0, 0, 5, 0, 0, 0, 0, 0, // row 6
    0, 0, 0, 0, 1, 0, 7, 8, 0, // row 7
    5, 0, 0, 0, 0, 9, 0, 0, 0, // row 8
    0, 0, 0, 0, 0, 0, 0, 4, 0, // row 9
];
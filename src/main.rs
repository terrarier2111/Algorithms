#![feature(int_roundings)]
#![feature(const_mut_refs)]
#![feature(const_maybe_uninit_write)]

mod sorting;
mod encrypt;
mod break_encrypt;
pub mod util;
mod search;
mod sudoku;
mod compression;

use std::collections::HashMap;
use std::mem::transmute;
use std::slice::Iter;
use std::collections::hash_map::Values;
use std::convert::TryInto;

use compression::huffman::{self, decode};

use crate::sudoku::print_constants;
use crate::sudoku::verifier::verify;

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
    
    
    
    
    let test_val = "abcdefghhhh";
    let encoded = huffman::encode(test_val).unwrap();
    println!("encoded: {:?}", encoded);
    let decoded = decode(encoded);
    assert_eq!(&decoded, test_val);
    println!("success!!!");
    
    
    
    
    
    
    /*let mut example = EXAMPLE_SUDOKU_BOARD_8.clone();
    sudoku::solver::solve(&mut example);
    let ex_print = unsafe { transmute::<_, [[u8; 9]; 9]>(example.clone()) };
    for row in ex_print {
        println!("{:?}", row);
    }
    assert!(verify(&example));
    println!("tz: {}", (1_usize << 8).trailing_zeros() as usize);
    print_constants();*/
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

const EXAMPLE_SUDOKU_BOARD_2: &[u8; 81] = &[
    0, 0, 1, 0, 0, 2, 3, 0, 4,
    4, 0, 0, 5, 0, 6, 0, 0, 1,
    0, 0, 0, 4, 0, 7, 0, 8, 6,
    0, 0, 0, 1, 0, 8, 2, 0, 7,
    0, 0, 0, 0, 0, 0, 0, 0, 0,
    6, 0, 9, 3, 0, 4, 0, 0, 0,
    1, 8, 0, 9, 0, 3, 0, 0, 0,
    9, 0, 0, 7, 0, 1, 0, 0, 5,
    3, 0, 2, 6, 0, 0, 1, 0, 0,
];

const EXAMPLE_SUDOKU_BOARD_3: &[u8; 81] = &[
    0, 0, 1, 0, 2, 0, 0, 0, 3,
    0, 0, 0, 0, 1, 0, 4, 0, 0,
    5, 0, 6, 0, 0, 7, 0, 0, 0,
    7, 0, 0, 4, 0, 0, 0, 0, 0,
    0, 3, 0, 0, 0, 0, 0, 8, 0,
    0, 0, 0, 0, 0, 9, 0, 0, 1,
    0, 0, 0, 5, 0, 0, 6, 0, 8,
    0, 0, 9, 0, 8, 0, 0, 0, 0,
    2, 0, 0, 0, 9, 0, 7, 0, 0,
];

// solvable
const EXAMPLE_SUDOKU_BOARD_4: &[u8; 81] = &[
    0, 0, 1, 2, 0, 0, 0, 0, 3,
    0, 0, 0, 0, 0, 4, 0, 0, 5,
    0, 0, 0, 6, 0, 0, 0, 7, 0,
    0, 3, 0, 0, 0, 6, 0, 2, 0,
    8, 0, 5, 0, 1, 0, 4, 0, 6,
    0, 7, 0, 4, 0, 0, 0, 9, 0,
    0, 6, 0, 0, 0, 9, 0, 0, 0,
    2, 0, 0, 7, 0, 0, 0, 0, 0,
    4, 0, 0, 0, 0, 1, 8, 0, 0,
];

const EXAMPLE_SUDOKU_BOARD_5: &[u8; 81] = &[
    0, 0, 1, 2, 0, 0, 3, 0, 0,
    0, 4, 0, 0, 3, 0, 0, 0, 5,
    0, 0, 0, 6, 0, 0, 7, 0, 8,
    0, 0, 8, 0, 0, 3, 0, 0, 0,
    1, 0, 0, 0, 0, 0, 0, 0, 4,
    0, 0, 0, 7, 0, 0, 9, 0, 0,
    5, 0, 6, 0, 0, 8, 0, 0, 0,
    8, 0, 0, 0, 1, 0, 0, 7, 0,
    0, 0, 3, 0, 0, 9, 2, 0, 0,
];

// solvable
const EXAMPLE_SUDOKU_BOARD_6: &[u8; 81] = &[
    0, 0, 1, 0, 2, 0, 0, 0, 3,
    0, 0, 0, 0, 3, 0, 0, 0, 4,
    0, 5, 0, 6, 0, 0, 0, 0, 0,
    0, 0, 0, 7, 0, 0, 0, 5, 8,
    0, 0, 9, 0, 0, 0, 1, 0, 0,
    6, 3, 0, 0, 0, 2, 0, 0, 0,
    0, 0, 0, 0, 0, 5, 0, 2, 0,
    4, 0, 0, 0, 8, 0, 0, 0, 0,
    1, 0, 0, 0, 9, 0, 7, 0, 0,
];

const EXAMPLE_SUDOKU_BOARD_7: &[u8; 81] = &[
    1, 0, 2, 3, 0, 0, 0, 0, 4,
    0, 5, 0, 0, 0, 0, 0, 0, 2,
    0, 0, 0, 6, 0, 0, 7, 0, 0,
    0, 8, 0, 0, 2, 0, 0, 0, 0,
    0, 0, 7, 0, 0, 0, 6, 0, 0,
    0, 0, 0, 0, 4, 0, 0, 1, 0,
    0, 0, 6, 0, 0, 9, 0, 0, 0,
    4, 0, 0, 0, 0, 0, 0, 3, 0,
    2, 0, 0, 0, 0, 8, 5, 0, 1,
];

const EXAMPLE_SUDOKU_BOARD_8: &[u8; 81] = &[
    0, 7, 0, 0, 0, 4, 0, 0, 2,
    0, 0, 1, 0, 3, 0, 0, 4, 0,
    0, 0, 0, 5, 0, 0, 1, 0, 0,
    0, 4, 0, 0, 0, 3, 0, 0, 8,
    0, 0, 3, 0, 0, 0, 7, 0, 0,
    1, 0, 0, 6, 0, 0, 0, 9, 0,
    0, 0, 4, 0, 0, 1, 0, 0, 0,
    0, 2, 0, 0, 7, 0, 8, 0, 0,
    5, 0, 0, 9, 0, 0, 0, 6, 0,
];
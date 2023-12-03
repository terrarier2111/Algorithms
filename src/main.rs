#![feature(int_roundings)]

mod sorting;
mod encrypt;
mod break_encrypt;
pub mod util;
mod search;
mod sudoku;

use std::collections::HashMap;
use std::slice::Iter;
use std::collections::hash_map::Values;
use std::convert::TryInto;

fn main() {
    let mut unsorted = [3, 7, 34, 937, 543, 63, 8427, 0, 1, 934, 1, 47427, 245];
    println!("{:?}", unsorted);
    sorting::insertion::sort(&mut unsorted);
    println!("sorted: {:?}", unsorted);
    let sorted = [1, 2, 3, 4, 5, 6, 7, 9];
    println!("found: {:?}", search::binary::search_length_limited_nearest(&sorted, /*3*/8/*100*/, sorted.len()));
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
}

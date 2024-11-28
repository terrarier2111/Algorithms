#![feature(int_roundings)]
#![feature(const_mut_refs)]
#![feature(const_maybe_uninit_write)]

mod sorting;
mod encrypt;
mod break_encrypt;
pub mod util;
mod search;
mod sudoku;
mod rng;
mod hash;
mod compression;
mod pathfinding;
mod prime_finder;

use std::collections::HashMap;
use std::mem::transmute;
use std::slice::Iter;
use std::collections::hash_map::Values;
use std::convert::TryInto;
use std::hash::Hasher;

use compression::huffman;
use hash::h1::H1;
use rng::acorn::AcornRng;
use rng::lcg::LCGGenerator64;
use rng::xor_shift::XorShiftPRng64;
use rng::Rng;

use crate::hash::h2::H2;
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
    /*let mut example = EXAMPLE_SUDOKU_BOARD_8.clone();
    sudoku::solver::solve(&mut example);
    let ex_print = unsafe { transmute::<_, [[u8; 9]; 9]>(example.clone()) };
    for row in ex_print {
        println!("{:?}", row);
    }
    assert!(verify(&example));
    println!("tz: {}", (1_usize << 8).trailing_zeros() as usize);
    print_constants();*/


    let test_val = "abcdefghhhh";
    let encoded = huffman::encode(test_val).unwrap();
    println!("encoded: {:?}", encoded);
    let decoded = huffman::decode(encoded);
    assert_eq!(&decoded, test_val);
    println!("success!!!");


    // RNG
    let mut xor_rng = XorShiftPRng64::new();
    let mut lcg_rng = LCGGenerator64::new();
    let mut acorn_rng = AcornRng::new();
    for _ in 0..100 {
        let xor_val = xor_rng.rand_u64();
        let lcg_val = lcg_rng.gen_u64();
        let acorn_val = acorn_rng.gen_u64();
        //let ratio = (u64::MAX / f64::MAX.round() as u64);
        //let c_val = (val / ratio) as f64 / f64::MAX;
        println!("XOR rand: {}", xor_val);
        println!("LCG rand: {}", lcg_val);
        println!("Acorn rand: {}", acorn_val);
    }
    println!("EA xor:");
    entropy_analysis(xor_rng);
    // FIXME: the lcg generator only has ~ 45% ones not 50% as it should
    println!("EA lcg:");
    entropy_analysis(lcg_rng);
    // FIXME:the acorn generator only has ~ 32% ones not 50% as it should
    println!("EA acorn:");
    entropy_analysis(acorn_rng);


    hash(64);
    hash(63);
    hash(20004);
    hash(0);
    let mut xor_rng = XorShiftPRng64::new();
    let val_cnt = 1000;
    let mut vals = vec![];
    let mut dist = 0;
    for _ in 0..val_cnt {
        let mut hasher = H2::new();
        hasher.write_u64(xor_rng.gen_u64());
        vals.push(hasher.finish());
        dist += ((u64::MAX / 2) as i128) - (hasher.finish() as i128);
    }
    println!("dist {}", dist);
}

fn hash(val: u64) {
    let mut hasher = H2::new();
    hasher.write_u64(val);
    println!("hashed: {}", hasher.finish());
    println!("hash: {}", {
        let mut result = String::new();
        for i in 0..64 {
            result.push(if hasher.finish() & (1 << i) != 0 { '1' } else { '0' });
        }
        result
    });
}

fn entropy_analysis(mut rng: impl Rng) {
    let mut bits = [0; u64::BITS as usize];
    let mut values = vec![];
    let mut exists = HashMap::new();
    let mut ones = 0;
    let mut pairs = [0; 4];
    let vals = 10000000;
    for _ in 0..vals {
        let val = rng.gen_u64();
        values.push(val);
        exists.entry(val).and_modify(|val: &mut u64| *val += 1).or_default();
        ones += val.count_ones();
        for bit in 0..(u64::BITS as usize) {
            bits[bit] += ((1 << bit) & val) >> bit;
        }
        for pair in 0..(u64::BITS as usize / 2) {
            let off = pair * 2;
            let val = ((3 << off) & val) >> off;
            pairs[val as usize] += 1;
        }
    }
    println!("ones: {}%", (ones as f64 / (values.len() * u64::BITS as usize) as f64 * 100.0));
    for bit in 0..(u64::BITS as usize) {
        if bits[bit] > (vals / 2 + vals / 2 / 10) || bits[bit] < (vals / 2 - vals / 2 / 10) {
            println!("Sussy bit {bit}: got count {}, expected {}", bits[bit], vals / 2);
        }
    }
    for pair in 0..4 {
        if pairs[pair] > (vals * 8 + vals * 8 / 10) || pairs[pair] < (vals * 8 - vals * 8 / 10) {
            println!("Sussy pair {}: got count {}, expected {}", if pair == 0 { "00" } else if pair == 1 { "10" } else if pair == 2 { "01" } else { "11" }, pairs[pair], vals * 8);
        }
    }
    let mut dups = [0; 10];
    for entry in exists.iter() {
        if *entry.1 > 1 {
            dups[(*entry.1).min(10) as usize] += 1;
        }
    }
    for (dups, cases) in dups.into_iter().enumerate() {
        if *cases > 0 {
            println!("Got {cases} cases of {dups} duplicates");
        }
    }
    // FIXME: analyse bit seqences
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
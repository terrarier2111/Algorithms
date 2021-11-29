/*use std::collections::HashMap;
use std::slice::Iter;
use std::collections::hash_map::Values;

fn main() {
    println!("Hello, world!");
    let unsorted = [3, 7, 34, 937, 543, 63, 8427];
    println!("{:?}", unsorted);
    sort(&unsorted);
    println!("{:?}", unsorted);
}


/// Sorts the given array in ascending order.
fn sort(src: &[i32]) -> Box<[i32]> {
    let mut lookup = HashMap::new();
    for i in src {
        lookup.insert(i,i.to_string());
    }
    let mut buckets: [Vec<i32>; 10] = [
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];
    let mut buckets2 = buckets.clone();
    let longest = longest(lookup.values());
    inner_sort(src.iter(), &mut buckets, &lookup, longest - 1 - 0);
    for i in 2..longest + 1 {
        for bucket in 0..10 {
            inner_sort(buckets[bucket].iter(), &mut buckets2, &lookup, longest - i);
        }
        let tmp = buckets;
        buckets = buckets2;
        buckets2 = tmp;
        for bucket in buckets2.iter_mut() {
            bucket.clear();
        }
    }
    let mut ret = Vec::with_capacity(src.len());
    for bucket in buckets {
        for num in bucket {
            ret.push(num);
        }
    }
    ret.into_boxed_slice()
}

fn inner_sort(src: Iter<'_, i32>, dst: &mut [Vec<i32>; 10], lookup: &HashMap<&i32, String>, lookup_idx: usize) {
    for i in src {
        let str = lookup.get(i).unwrap();
        let num = get_char_or_def(str, lookup_idx, '0') as u32 - '0' as u32;
        println!("idx {} str {} num {}", lookup_idx, str, num);
        dst[num as usize].push(*i);
    }
}

fn get_char_or_def(str: &String, idx: usize, def: char) -> char {
    str.chars().nth(idx).map_or_else(|| def, |x| x)
}

// mo c2
// di c3
// mi c2


// Notengebung:
/*
Kursarbeiten: 1: wertung: 1/3
Mündlich + Hüs: wertung: 2/3
Kursarbeit wird zur hälfte am pc geschrieben
Film oder ausarbeitung
*/
´
*/

mod sorting;
mod encrypt;
mod break_encrypt;
pub mod util;
mod search;

use std::collections::HashMap;
use std::slice::Iter;
use std::collections::hash_map::Values;
use std::convert::TryInto;

fn main() {
    let unsorted = [3, 7, 34, 937, 543, 63, 8427, 0, 1, 934, 1, 47427, 245];
    println!("{:?}", unsorted);
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
    println!("found position: {}", search::binary::search(to_query_through, 30));
}

/// Sorts the given array in ascending order.
fn sort(src: &[i32]) -> Box<[i32]> {
    let mut lookup = HashMap::new();
    for i in src {
        lookup.insert(i,i.to_string());
    }
    let mut buckets: [Vec<i32>; 10] = [
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];
    let mut result = buckets.clone();
    let longest = longest(lookup.values());
    for x in src.iter() {
        let str_ver = lookup.get(x).unwrap();
        let mut vec = buckets.get_mut(char_as_num(str_ver.chars().nth(str_ver.len() - 1).unwrap()) as usize).unwrap();
        vec.push(*x);
    }
    for i in 1..longest {
        for bucket in ((0)..10).rev() {
            for x in buckets.get(bucket).unwrap().iter() {
                let str_ver = lookup.get(x).unwrap();
                let mut vec = result.get_mut(char_as_num(str_ver.chars().nth(str_ver.len() - 1 - i).unwrap_or('0')) as usize).unwrap();
                vec.push(*x);
            }
        }
        for bucket in result.iter() {
            println!("buck {:?}", bucket);
        }
        let tmp = buckets;
        buckets = result;
        result = tmp;
        for bucket in result.iter_mut() {
            bucket.clear();
        }
    }
    let mut ret = Vec::with_capacity(src.len());
    for bucket in buckets.iter().rev() {
        for num in bucket {
            ret.push(*num);
        }
    }
    ret.into_boxed_slice()
}

fn char_as_num(input: char) -> u8 {
    (input as u32 - '0' as u32) as u8
}

fn longest(strs: Values<'_, &i32, String>) -> usize {
    let mut longest = 0;
    for str in strs {
        if str.len() > longest {
            longest = str.len();
        }
    }
    longest
}
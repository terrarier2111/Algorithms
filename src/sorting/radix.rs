use std::collections::hash_map::Values;
use std::collections::HashMap;

/// Sorts the given array in ascending order.
pub(crate) fn sort(src: &[i32]) -> Box<[i32]> {
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

#[inline]
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
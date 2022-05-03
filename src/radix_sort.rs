use std::collections::HashMap;

/// Sorts the given array in ascending order.
pub fn sort(src: &[i32]) -> Box<[i32]> {
    let mut lookup = HashMap::new();
    for i in src {
        lookup.insert(*i, i.to_string().into_bytes().into_boxed_slice());
    }
    let longest = longest(&lookup);
    let mut buckets = [
        vec![]/*.into_boxed_slice()*/,
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
    for x in src.iter() {
        let str_ver = lookup.get(x).unwrap();
        let mut bucket = buckets.get_mut(char_as_num(str_ver[str_ver.len() - 1] as char) as usize).unwrap();
        bucket.push(*x);
    }

    for i in 1..longest {
        for bucket in (0..10).rev() {
            for x in buckets.get(bucket).unwrap().iter() {
                let str_ver = lookup.get(x).unwrap();
                let idx = str_ver.len() as isize - 1 - i as isize;
                let elem = if idx >= 0 {
                    char_as_num(str_ver[idx as usize] as char) as usize
                } else {
                    0
                };
                let mut bucket = result.get_mut(elem).unwrap();
                bucket.push(*x);
            }
        }
        for bucket in result.iter() {
            println!("bucket {:?}", bucket);
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

fn longest(strs: &HashMap<i32, Box<[u8]>>) -> usize {
    let mut longest = 0;
    for str in strs.values() {
        if str.len() > longest {
            longest = str.len();
        }
    }
    longest
}
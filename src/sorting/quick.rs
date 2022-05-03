/*pub fn sort(src: &mut [i32]) {
    // recursive_sort(src, src.len() / 2, src.len() / 2);
    while true {
        let mut pivot = src[src.len() - 1]; // FIXME: Does this need to be mut?
        let mut smaller = None;
        let mut greater = None;
        let mut i = 0;
        let mut j = src.len() - 2;
        while i < j {
            if smaller.is_none() {
                if src[j] < pivot {
                    if greater.is_some() {
                        let greater = greater.take();
                    } else {
                        smaller = Some(src[j]);
                    }
                } else {
                    j -= 1;
                }
            }
        }
    }
}*/

/*
fn recursive_sort(src: &mut [i32], piviot_idx: usize, len: usize) {
    let start = piviot_idx - len;
    let end = piviot_idx + len;
    for x in start..piviot_idx {

    }
}*/

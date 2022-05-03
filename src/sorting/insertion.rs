pub fn sort(src: &mut [usize]) {
    for i in 1..src.len() {
        let curr = src[i];
        for j in 0..i { // FIXME: Use a binary search to the closest element
            if src[j] > curr {
                println!("first replacement {} max: {}", j, i);
                for k in (j..i).rev() {
                    src[k + 1] = src[k];
                    println!("inner loop: {}", k);
                }
                src[j] = curr;
                println!("after loop vals: {:?}", src);
                break;
            }
        }
    }
}

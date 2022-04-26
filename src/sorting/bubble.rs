pub fn sort/*<T: PartialOrd, const MIN: T>*/(src: &mut [i32]) {
    // go over over the entire array
    for i in 0..src.len() {
        let mut max = i32::MIN;
        let mut max_idx = 0;
        // go over over the array starting at i (to skip all the things we already sorted in previous steps)
        for k in i..src.len() {
            // check if the element at index k is greater than our currently found max
            if src[k] >= max {
                // if so, replace the current max with the current element
                max = src[k];
                max_idx = k;
            }
        }
        // swap the elements at index i and index max_idx
        src[max_idx] = src[i];
        src[i] = max;
    }
}
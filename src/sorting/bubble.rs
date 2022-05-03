pub fn sort(src: &mut [i32]) {
    for i in 0..src.len() {
        for x in 1..(src.len() - i) {
            if src[x - 1] > src[x] {
                let tmp = src[x];
                src[x] = src[x - 1];
                src[x - 1] = tmp;
            }
        }
    }
}
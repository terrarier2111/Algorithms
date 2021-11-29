pub fn sort(src: &mut [i32]) -> Box<&[i32]> {
    /*for i in src {
        let elem = src[i as usize];
        for j in 0..src.len() {
            let curr = src[j as usize];
            if
        }
    }*/

    Box::new(&*src)
}
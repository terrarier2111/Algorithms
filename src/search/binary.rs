pub fn search(container: &[usize], target: usize) -> usize { /// returns a position
    let mut curr_pos = container.len() / 2;
    let mut step_size = container.len() / 4;
    let mut tried_one = false;
    loop {
        let curr = container[curr_pos];
        if curr > target {
            if step_size < 2 && tried_one {
                step_size = 0;
                curr_pos -= 1;
            } else if step_size < 2 {
                tried_one = true;
            }
            curr_pos += step_size;
            step_size /= 2;
        } else if curr < target {
            if step_size < 2 && tried_one {
                step_size = 0;
                curr_pos -= 1;
            } else if step_size < 2 {
                tried_one = true;
            }
            curr_pos -= step_size;
            step_size /= 2;
        } else {
            return curr_pos;
        }
    }
}
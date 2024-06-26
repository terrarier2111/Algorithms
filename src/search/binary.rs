/// Returns the index where the target element is.
///
/// Returns the next smaller index on failure.
pub fn search_nearest(container: &[usize], target: usize) -> usize {
    search_length_limited_nearest(container, target, container.len())
}

/// Returns the index where the target element is.
///
/// Returns the next smaller index on failure.
pub fn search_length_limited_nearest<T: PartialOrd>(container: &[T], target: T, length: usize) -> usize {
    let mut curr_pos = length / 2;
    let mut step_size = length / 4;
    let mut adapted = false;
    loop {
        if container[curr_pos] > target {
            if step_size == 0 {
                if !adapted {
                    adapted = true;
                    step_size += 1;
                } else {
                    if container[curr_pos] > target {
                        return curr_pos - 1;
                    }
                    return curr_pos;
                }
            }
            curr_pos -= step_size;
            step_size /= 2;
        } else if container[curr_pos] < target {
            if step_size == 0 {
                if !adapted {
                    adapted = true;
                    step_size += 1;
                } else {
                    if container[curr_pos] > target {
                        return curr_pos - 1;
                    }
                    return curr_pos;
                }
            }
            curr_pos += step_size;
            step_size /= 2;
        } else {
            return curr_pos;
        }
    }
}
pub fn binary_search_ints(slice: &[i32], target: i32) -> isize {
    if slice.is_empty() {
        return -1;
    }
    let mut left = 0;
    let mut right = slice.len() - 1;
    let mut result = -1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if slice[mid] == target {
            result = mid as isize;
            if mid == 0 {
                break;
            }
            right = mid - 1;
        } else if slice[mid] < target {
            left = mid + 1;
        } else {
            if mid == 0 {
                break;
            }
            right = mid - 1;
        }
    }
    result
}

pub fn quicksort_like_merge(list: &mut Vec<i32>, left: usize, right: usize, end: usize) {
    if left >= right || right >= end { return; }
    if list[right - 1] < list[right] { return; }
    let left_center = (left + right) / 2;
    let first_right_not_piv = find_first_not_less_than(list, right, end, list[left_center]);
    rotate(list, left_center, right, first_right_not_piv);
    let post_rot_count = first_right_not_piv - right;
    let post_rot_start = post_rot_count + left_center;
    quicksort_like_merge(list, left, left_center, post_rot_start);
    quicksort_like_merge(list, post_rot_start + 1, first_right_not_piv, end);
}

fn rotate(list: &mut Vec<i32>, old_first: usize, new_first: usize, end: usize){
    reverse(list, old_first, new_first);
    reverse(list, new_first, end);
    reverse(list, old_first, end);
}

fn reverse(list: &mut Vec<i32>, first: usize, end: usize){ 
    let mut left = first;
    let mut right = end - 1;
    while left < right {
        list[left] = list[left] ^ list[right];
        list[right] = list[right] ^ list[left];
        list[left] = list[left] ^ list[right];
        left += 1;
        right -= 1;
    }
}

fn find_first_not_less_than(list: &mut Vec<i32>, start: usize, end: usize, val: i32) -> usize {
    let mut left = start;
    let mut right = end;

    while left < right {
        let center = (left + right) / 2;
        if list[center] < val { left = center + 1; }
        else { right = center; }
    }

    return left;
}

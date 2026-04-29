pub fn power_merge(list: &mut Vec<i32>) {
    let mut sub_size: usize = 1;
    while sub_size < list.len() {
        build_larger_sublists(list, sub_size);
        sub_size *= 2;
    }
    for num in list.clone() {
        print!("{}, ", num.to_string());
    }
}

fn build_larger_sublists(list: &mut Vec<i32>, cur_sub_size: usize){
    let mut left = 0;
    while left + cur_sub_size < list.len() {
        let right = left + cur_sub_size;
        let end = std::cmp::min(right + cur_sub_size, list.len());
        merge(list, left, right, end);
        left = end;
    }
    if left != list.len() {
        merge(list, left - 2 * cur_sub_size, left, list.len());
    }
}

fn merge(list: &mut Vec<i32>, left: usize, right: usize, end: usize) {
    if left >= right || right >= end { return; }
    if list[right - 1] < list[right] { return; }
    let left_center = (left + right) / 2;
    let first_right_not_piv = find_first_not_less_than(list, right, end, list[left_center]);
    rotate(list, left_center, right, first_right_not_piv);
    let post_rot_count = first_right_not_piv - right;
    let post_rot_start = post_rot_count + left_center;
    merge(list, left, left_center, post_rot_start);
    merge(list, post_rot_start + 1, first_right_not_piv, end);
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

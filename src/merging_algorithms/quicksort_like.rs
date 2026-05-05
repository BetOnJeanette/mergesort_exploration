pub fn quicksort_like_merge(list: &mut Vec<i32>, left: usize, right: usize, end: usize) {
    if left >= right || right >= end { return; }
    if list[right - 1] < list[right] { return; }
    let (rotate_start, rotate_end) = select_piv_range(list, left, right, end);
    if rotate_start >= right {
        quicksort_like_merge(list, left, right, rotate_end);
    } else if rotate_end < right {
        quicksort_like_merge(list, rotate_start, right, end);

    } else {
        rotate(list, rotate_start, right, rotate_end);
        let post_rot_count = rotate_end - right;
        let post_rot_start = post_rot_count + rotate_start;
        quicksort_like_merge(list, left, rotate_start, post_rot_start);
        quicksort_like_merge(list, post_rot_start, rotate_end, end);
    }
}

fn rotate(list: &mut Vec<i32>, old_first: usize, new_first: usize, end: usize){
    reverse(list, old_first, new_first);
    reverse(list, new_first, end);
    reverse(list, old_first, end);
}

fn select_piv_range(list: &mut Vec<i32>, left: usize, right: usize, end: usize) -> (usize, usize) {
    let left_size = right - left;
    let right_size = end - right;
    if right_size < left_size {
        let right_center = (right + end) / 2;
        if list[right - 1] < list[right_center] { return (right, right_center); }
        let left_center = find_first_greater_than(list, left, right, list[right_center]);
        return (left_center, right_center + 1);
    } else {
        let left_center = (left + right) / 2;
        if list[left_center] < list[right] { return (left_center, right - 1); }
        let right_center = find_first_not_less_than(list, right, end, list[left_center]);
        return (left_center, right_center);
    }
}

fn reverse(list: &mut Vec<i32>, first: usize, end: usize){ 
    let mut left = first;
    let mut right = end - 1;
    while left < right {
        list.swap(left, right);
        left += 1;
        right -= 1;
    }
}

fn find_first_greater_than(list: &mut Vec<i32>, start: usize, end: usize, val: i32) -> usize {
    let mut left = start;
    let mut right = end;
    while left < right {
        let center = (left + right) / 2;
        if  list[center] <= val { left = center + 1; }
        else { right = center; }
    }

    return left;
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

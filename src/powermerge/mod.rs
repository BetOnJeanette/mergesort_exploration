use crate::merging_algorithms::quicksort_like::quicksort_like_merge;

pub fn power_merge(list: &mut Vec<i32>) {
    let mut sub_size: usize = 1;
    while sub_size < list.len() {
        build_larger_sublists(list, sub_size);
        sub_size *= 2;
    }
}

pub fn power_merge_top_down(list: &mut Vec<i32>) {
    let center = list.len() / 2;
    power_merge_top_down_helper(list, 0, center, list.len()); 
}

fn power_merge_top_down_helper(list: &mut Vec<i32>, left: usize, right: usize, end: usize){
    if left >= right || right >= end { return; }
    let left_center = (left + right) / 2;
    let right_center = (right + end) / 2;
    power_merge_top_down_helper(list, left, left_center, right);
    power_merge_top_down_helper(list, right, right_center, end);
}

fn build_larger_sublists(list: &mut Vec<i32>, cur_sub_size: usize){
    let mut left = 0;
    while left + cur_sub_size < list.len() {
        let right = left + cur_sub_size;
        let end = std::cmp::min(right + cur_sub_size, list.len());
        quicksort_like_merge(list, left, right, end);
        left = end;
    }
    if left != list.len() {
        quicksort_like_merge(list, left - 2 * cur_sub_size, left, list.len());
    }
}


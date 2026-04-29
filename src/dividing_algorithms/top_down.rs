use crate::merging_algorithms::Merger;

pub fn merge_sort(list: &mut Vec<i32>, merge: Merger) {
    let center = list.len() / 2;
    merge_sort_helper(list, 0, center, list.len(), merge); 
}

fn merge_sort_helper(list: &mut Vec<i32>, left: usize, right: usize, end: usize, merge: Merger){
    if left >= right || right >= end { return; }
    let left_center = (left + right) / 2;
    let right_center = (right + end) / 2;
    merge_sort_helper(list, left, left_center, right, merge);
    merge_sort_helper(list, right, right_center, end, merge);
    merge(list, left, right, end);
}

use crate::merging_algorithms::Merger;

fn merge_sort(list: &mut Vec<i32>, merge: Merger) {
    let mut sub_size: usize = 1;
    while sub_size < list.len() {
        build_larger_sublists(list, sub_size, merge);
        sub_size *= 2;
    }
}

fn build_larger_sublists(list: &mut Vec<i32>, cur_sub_size: usize, merge: Merger){
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

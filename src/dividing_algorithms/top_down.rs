pub fn merge_sort(list: &mut Vec<i32>) {
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

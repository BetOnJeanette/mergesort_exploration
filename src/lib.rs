pub mod merging_algorithms;
pub mod dividing_algorithms;

pub fn quicksort(list: &mut Vec<i32>){
    quicksort_helper(list, 0, list.len());
}

fn quicksort_helper(list: &mut Vec<i32>, start: usize, end: usize){
    if start >= end { return; }
    let mut left = start + 1; 
    let mut right = end - 1;
    let pivot = list[start];
    while left < right{
        while left < end && list[left] < pivot  { left += 1; }
        while right > start && list[right] > pivot { right -= 1; }
        if left < right { swap(list, left, right); }
    }
    if right > start && list[right] < pivot {
        swap(list, start, right);
    }
    quicksort_helper(list, start, right);
    quicksort_helper(list, right + 1, end);
}
fn swap(list: &mut Vec<i32>, idx_a: usize, idx_b: usize){
    if idx_a == idx_b { return; }
    list[idx_a] = list[idx_a] ^ list[idx_b];
    list[idx_b] = list[idx_b] ^ list[idx_a];
    list[idx_a] = list[idx_a] ^ list[idx_b];
}

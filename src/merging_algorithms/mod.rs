pub mod quicksort_like;
pub mod traditional;

pub type Merger = fn(list: &mut Vec<i32>, left: usize, right: usize, end: usize);

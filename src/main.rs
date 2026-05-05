use rand::prelude::*;
use power_merge::{dividing_algorithms::bottom_up, merging_algorithms::{quicksort_like::quicksort_like_merge, traditional}};

fn main() {
    let mut rng = rand::rng();
    let mut vec: Vec<i32> = (0..2_i32.pow(5)).collect();
    vec.shuffle(&mut rng);
    print!("Before: [");
    for item in vec.clone() {
        print!("{}, ", item);
    }
    println!("]");
    bottom_up::merge_sort(&mut vec, quicksort_like_merge);
    print!("after: [");
    for item in vec.clone() {
        print!("{}, ", item);
    }
    println!("]");
}


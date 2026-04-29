use rand::prelude::*;
use power_merge::{dividing_algorithms::bottom_up, merging_algorithms::quicksort_like::quicksort_like_merge};

fn main() {
    let mut rng = rand::rng();
    let mut vec: Vec<i32> = (0..2048).collect();
    vec.shuffle(&mut rng);
    print!("before: [",);
    for num in vec.clone() {
        print!("{}, ", num.to_string());
    }
    println!("]");
    bottom_up::merge_sort(&mut vec,quicksort_like_merge);
    print!("after: [",);
    for num in vec.clone() {
        print!("{}, ", num.to_string());
    }
    println!("]");
}


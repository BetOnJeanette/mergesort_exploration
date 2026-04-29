use rand::prelude::*;
use power_merge::powermerge;

fn main() {
    let mut rng = rand::rng();
    let mut vec: Vec<i32> = (0..2048).collect();
    vec.shuffle(&mut rng);
    print!("before: [",);
    for num in vec.clone() {
        print!("{}, ", num.to_string());
    }
    println!("]");
    powermerge::power_merge(&mut vec);
    print!("after: [",);
    for num in vec.clone() {
        print!("{}, ", num.to_string());
    }
    println!("]");
}


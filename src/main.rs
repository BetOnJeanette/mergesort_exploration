use rand::prelude::*;
use power_merge::powermerge;

fn main() {
    let mut rng = rand::rng();
    let mut vec: Vec<i32> = (0..2048).collect();
    vec.shuffle(&mut rng);
    powermerge::power_merge(&mut vec);
}


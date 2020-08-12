use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

const CHARACTERS: &[u8; 60] = b"abcdefghilkjapqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

pub fn generate_string(min: i32, max: i32) -> String {
    let mut rng = rand::thread_rng();
    let mut s = String::from("");

    for _ in min..=max {
        s.push(CHARACTERS.choose(&mut rng).map(|&c| c as char).unwrap())
    }

    s
}

fn main() {
	println!("Generating random strings of variable sizes!");
	for i in 1..=1000 {
		println!("{}", generate_string(1, i));
	}
}

use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

// constant array of characters that are legal options for the random engine
const CHARACTERS: &[u8; 60] = b"abcdefghilkjapqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

// function that generates our string with a specific length
fn generate_string(min: i32, max: i32) -> String {
    let mut rng = rand::thread_rng(); // this cache is probably not as effective as I think

	// loop n times, relative to parameters
    for _ in min..=max {
		// push n number of characters into the string. rng engine chooses from the array
        s.push(CHARACTERS.choose(&mut rng).map(|&c| c as char).unwrap())
    }

    s	// return the newly generated string
}

fn main() {
	println!("Generating random strings of variable sizes!");

	// loop 1000 times, hardcoded for now - could be defined by a cli arg etc
	for i in 1..=1000 {
		println!("{}", generate_string(1, i));
	}
}

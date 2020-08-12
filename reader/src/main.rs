use std::env;
use std::process;
use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    // Parse the cli args, error out and show usage if not ran correctly
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 2 {
        println!("Usage: {} file.txt", arguments[0]);
        process::exit(99);
    }

    // Access the target file
    let file_name: String = arguments[1].clone(); // grab the string from the arguments
    let res_file: Result<File, io::Error> = File::open(file_name);
    let f: File = match res_file { // match is basically the 'switch case' of rust.
		// the res_fule is of type Result, a default enum in Rust with Ok and Err
        Ok(file) => file, // return the file to 'f'
        Err(err) => {
            println!("Err: opening a file: {}", err);
            process::exit(99);
        },
    };

	// declare the count to be mutable so we can read/write to the value, right?
	// mutable seems like an inverted const.
	let mut count = 0;

    // read file line by line, hopefully it doesn't pull everything in memory at once
    let reader: BufReader<File> = BufReader::new(f);

	// loops the file, line by line through the buffered reader. Ideally, reader isn't the whole file in memory!
    for res_line in reader.lines() {
		// Might slow down the program a little without the output BUT shows progression
		if count % 1000000 == 0{
			println!("Hit milestone: {}", count);		
		}
		
		// "switch case" again on the Result enum, two otions Ok and Err only
        match res_line {
            Ok(_) => {
				// increment our count as we hit each line
				count += 1;
			},
            Err(err) => {
                println!("Err: {:?}", err);
                process::exit(99);
            },
        }
    }

	println!("Total words read: {}", count);
}

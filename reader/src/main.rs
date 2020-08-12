use std::env;
use std::process;
use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    // Parse the cli args
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 2 {
        println!("Usage: {} file.txt", arguments[0]);
        process::exit(99);
    }

    // Access the target file
    let file_name: String = arguments[1].clone();
    let res_file: Result<File, io::Error> = File::open(file_name);
    let f: File = match res_file {
        Ok(file) => file,
        Err(err) => {
            println!("Err: opening a file: {}", err);
            process::exit(99);
        },
    };

    // read file line by line, hopefully it doesn't pull everything in memory at once
	let mut count = 0;
    let reader: BufReader<File> = BufReader::new(f);
    for res_line in reader.lines() {
		// Might slow down the program a little without the output BUT shows progression
		if count % 1000000 == 0{
			println!("Hit milestone: {}", count);		
		}
		
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

use std::fs;

// struct for pid. They will know their pid, level (recursion depth) and their parent pid
struct Proc {
	pid: String,
	level: usize,
	parent_pid: String
}

// get children just creates the empty vector and starts off the recurisve loop on level 0
fn get_children(pid: &String) -> Vec<Proc> {
	let mut final_children: Vec<Proc> = Vec::new();

	// fire off the resursive function, passing in a ref to the vector and the starting pid + level
	find_children(pid, 0, &mut final_children);

	// return the now populated vector
	final_children
}

fn find_children(pid: &String, mut level: usize, final_children: &mut Vec<Proc>) {
	// read the processes, thanks h0mbre_ for this starting point
	let children_data = fs::read_to_string(format!("/proc/{}/task/{}/children", pid, pid)).unwrap();

	// create a collection/vector of all the identified children
	let children: Vec<String> = children_data.split_whitespace()
		.map(|x| x.to_string())
		.collect();

	// increment our level as we are likely going to go down a step next
	// i believe this will increment per recursive path, so it shouldn't go out of sync and be relative
	// to the current hierarchy it is crawling down... logical? 
	level += 1;

	for c in children.iter() {
		// for each child found, create a Proc object with our struct and populate
		let new_proc = Proc {
		    pid: c.clone(),
			level: level,
			parent_pid: pid.clone()
		};

		// push the object into our vector
		final_children.push(new_proc);

		// continue recursive search for our 'current' pid, looking for it's children
		find_children(c, level, final_children);
	}
}

fn main() {
	println!("Gather processes and children");
	println!(" -- Output is a mess atm, need to structure");

	let procs = get_children(&String::from("1"));

	// TODO:
	// Loop through and make sure the hierarchy looks correct.
	// As they are in a 'sort of' clean order we can probably just do clean output to help readability
	for p in procs.iter() {
		println!("Parent: {}, pid: {}, level: {}", p.parent_pid, p.pid, p.level);
	}	
}

use std::io::{stdin};

fn add(int_stack: &mut Vec<i64>) -> i32 {
	let n1 = match int_stack.pop() {
		None =>  {
			println!("Stack underflow");
			return -1
		}
		Some(n) => n,
	};
	let n2 = match int_stack.pop() {	
		None =>  {
			println!("Stack underflow");
			return -1;
		}
		Some(n) => n,
	};
	int_stack.push(n1+n2);
	return 0
}

fn pop(int_stack: &mut Vec<i64>) -> i32 {
	if int_stack.is_empty() {
		println!("Stack underflow");
		return -1
	}
	int_stack.pop();
	return 0;
}

fn process_input(input_array: Vec<&str>, int_stack: &mut Vec<i64>) -> i32 {
	for input in input_array {
		match input.parse::<i64>() {
			Ok(n) => { // if number then push it to the stack
				int_stack.push(n)
			}
			Err(_) => { // else check dictionnary
				match input {
					// dictionnary
					"show" => {
						println!("{:?}", int_stack);
					}
					"+" => {
						if add(int_stack) == -1 {
							return -1
						}
					}
					"drop" => {
						if pop(int_stack) == -1 {
							return -1
						}
					}
					"bye" => return 1,
					_ => {
						println!("{} : not known", input);
						return -1
					}	
				}
			}
		}
	}
	return 0;
}

fn main() {
	let mut int_stack: Vec<i64> = Vec::new();
	let mut line = String::new();
	println!("Welcome to rForth\ntype 'bye' to exit");
	loop {
		stdin().read_line(&mut line)
			.expect("err: read_line()");
		line = line.trim().to_string();
		let input_array = line.split(' ').collect();
		match process_input(input_array, &mut int_stack) {
			0 => {
				println!(" ok.");
			}
			1 => return,
			-1 => (),
			_ => (),
		}
		line.clear();
	}
}

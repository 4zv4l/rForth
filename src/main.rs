use std::io::{stdin};
	
fn process_input(input_array: Vec<&str>, int_stack: &mut Vec<i64>) -> bool {
	for input in input_array {
		match input.parse::<i64>() {
			Ok(n) => { // if number then push it to the stack
				int_stack.push(n)
			}
			Err(_) => { // else check dictionnary
				match input {
					"show" => println!("{}", input),
					"bye" => return false,
					_ => println!("word : {} : not known", input)
				}
			}
		}
	}
	return true;
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
			true => (),
			false => return
		}
		println!("{:?}", int_stack);
		line.clear();
	}
}

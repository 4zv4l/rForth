use std::io::{stdin, BufReader, BufRead};
use std::fs::File;

fn mul(int_stack: &mut Vec<i64>) -> i32 {
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
	int_stack.push(n1*n2);
	return 0
}

fn div(int_stack: &mut Vec<i64>) -> i32 {
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
	if n2 == 0 {
		println!("Division by zero");
		return -1
	}
	int_stack.push(n1/n2);
	return 0
}

fn sub(int_stack: &mut Vec<i64>) -> i32 {
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
	int_stack.push(n1-n2);
	return 0
}

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
					"-" => {
						if sub(int_stack) == -1 {
							return -1
						}
					}
					"*" => {
						if mul(int_stack) == -1 {
							return -1
						}
					}
					"/" => {
						if div(int_stack) == -1 {
							return -1
						}
					}
					"drop" => {
						if pop(int_stack) == -1 {
							return -1
						}
					}
					"bye" => return 1,
					_ => { // else doesn't know the word
						println!("{} : not known", input);
						return -1
					}	
				}
			}
		}
	}
	return 0;
}

fn stdin_interpreter(mut fd: Box<dyn BufRead>) {
	let mut int_stack: Vec<i64> = Vec::new();
	let mut line = String::new();
	loop {
		fd.read_line(&mut line)
			.expect("err: read_line()");
		line = line.trim().to_string();
		let input_array = line.split(' ').collect();
		match process_input(input_array, &mut int_stack) {
			0 => println!(" ok."), // succeeded
			1 => return, // bye
			-1 => (), // word not found
			_ => (),
		}
		line.clear();
	}
}

fn file_interpreter(mut fd: Box<dyn BufRead>) {
	let mut int_stack: Vec<i64> = Vec::new();
	let mut line = String::new();
	while fd.read_line(&mut line).unwrap() > 0 {
		line = line.trim().to_string();
		let input_array = line.split(' ').collect();
		match process_input(input_array, &mut int_stack) {
			1 => return, // bye
			-1 => (), // word not found
			_ => (),
		}
		line.clear();
	}
}

fn main() {
	// check argument to either read from a file or using stdin
	let args: Vec<String> = std::env::args().collect();
	match args.len() {
		1 => {
			println!("Welcome to rForth\ntype 'bye' to exit");
			let fd = Box::new(BufReader::new(stdin()));
			stdin_interpreter(fd)
		}
		2 => {
			let f = File::open(&args[1]).expect("err: Cannot open this file");
			let fd = Box::new(BufReader::new(f));
			file_interpreter(fd)
		}
		_ => {
			println!(
				"{}{}{}",
				"rForth usage :",
				"\n\trForth\trForth interpreter using stdin",
				"\n\trForth <file\t>rForth interpretrer using a file as input")
		}
	};
}

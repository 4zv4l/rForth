use std::io::{stdin, BufReader, BufRead};
use std::fs::File;

// take 2 top numbers from the stack and multiply them
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
	return 0;
}

// take 2 top numbers from the stack and divide them
// take care of 0 division
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

// take 2 top numbers from the stack and substitute them
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

// take 2 top numbers from the stack and addition them
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

// remove the top number of the stack
fn pop(int_stack: &mut Vec<i64>) -> i32 {
	if int_stack.is_empty() {
		println!("Stack underflow");
		return -1
	}
	int_stack.pop();
	return 0;
}

fn start_compile(compile_flag: &mut bool) {
	*compile_flag = true;
}
fn compile(compiled_words: &mut String, s: String) {
	let b = s + " "; // to separate words using space

	// _TODO_ code to redifine words if already exist

	compiled_words.push_str(&b);
}
fn stop_compile(compile_flag: &mut bool, compiled_words: &mut String) {
	*compile_flag = false;
	compiled_words.push_str("<<<>>>");
}

// get a word and return it's code
fn get_from_words(input: String, compiled_words: &mut String) -> Vec<&str> {
	let mut words: Vec<&str> = compiled_words.split("<<<>>>").collect(); // for each words
	words.retain(|&s| *s != *"");
	for word in words { // for each element in the word
		let mut key: Vec<&str> = word.split(' ').collect();
		key.retain(|&s| *s != *"");
		if input == key[0] {
			key.remove(0); // to avoid recursion remove the word itself from its code
			return key
		}
	}
	return Vec::new();
}

// process each words given by a file or a user
fn process_input(input_array: Vec<&str>, int_stack: &mut Vec<i64>, compile_flag: &mut bool, compiled_words: &mut String) -> i32 {
	for input in input_array {
		if *compile_flag { // enter in compile mode
			if input == ";" { // exit compile mode
				stop_compile(compile_flag, compiled_words);
			} else {
				compile(compiled_words, input.to_string());
			}
		} else {
			match input.parse::<i64>() {
				Ok(n) => { // if number then push it to the stack
					int_stack.push(n)
				}
				Err(_) => { // else check dictionnary
					match input {
						// dictionnary
						".s" => {
							let stack = int_stack.clone();
							print!("<{}> ", int_stack.len());
							for e in stack {
								print!("{} ", e);
							}
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
						"cr" => {
							println!("");
						}
						":" => { // set compile mode
							start_compile(compile_flag);
						}
						";" => {
							println!("not in compile mode");
							return -1;
						}

						// _TODO_ adding more builtin words here

						"words" => {
							// show words
							let mut words: Vec<&str> = compiled_words.split("<<<>>>").collect();
							words.retain(|&s| *s != *"");
							for word in words {
								println!(": {} ;", word);
							}
						}
						"bye" => return 1,
						_ => { // else check if it's in compiled words
							if compiled_words.contains(input) {
								let mut words = compiled_words.clone();
								let word: Vec<&str> = get_from_words(input.to_string(), &mut words);
								process_input(word, int_stack, compile_flag, &mut *compiled_words);
							} else { // doesn't know the word
								println!("{} : not known", input);
								return -1
							}
						}
					}
				}
			}
		} 
	}
	return 0;
}

// taking input from the user
fn stdin_interpreter(mut fd: Box<dyn BufRead>) {
	let mut int_stack: Vec<i64> = Vec::new();
	let mut line = String::new();
	let mut compile_flag: bool = false;
	let mut compiled_words = String::new();
	loop {
		fd.read_line(&mut line)
			.expect("err: read_line()");
		line = line.trim().to_string();
		let mut input_array: Vec<&str> = line.split(' ').collect();
		// remove all spaces left
		input_array.retain(|&s| *s != *"");
		match process_input(input_array, &mut int_stack, &mut compile_flag, &mut compiled_words) {
			0 => {
				if !compile_flag { // don't show while making words
					println!(" ok."); // succeeded
				}
			}
			1 => return, // bye
			-1 => (), // word not found
			_ => (),
		}
		line.clear();
	}
}

// reading from a file
fn file_interpreter(mut fd: Box<dyn BufRead>) {
	let mut int_stack: Vec<i64> = Vec::new();
	let mut line = String::new();
	let mut compile_flag: bool = false;
	let mut compiled_words = String::new();
	while fd.read_line(&mut line).unwrap() > 0 {
		line = line.trim().to_string();
		let input_array = line.split(' ').collect();
		match process_input(input_array, &mut int_stack, &mut compile_flag, &mut compiled_words) {
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

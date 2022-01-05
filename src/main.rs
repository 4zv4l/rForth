use std::io::{stdin, BufReader, BufRead, Read};
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
fn drop(int_stack: &mut Vec<i64>) -> i32 {
    if int_stack.is_empty() {
        println!("Stack underflow");
        return -1
    }
    int_stack.pop();
    return 0;
}

// get a key input from the user and push the ascii value to the stack
fn key(int_stack: &mut Vec<i64>) -> i32 {
    // set terminal to raw mode to get key input
    let stdin = stdin();
    let mut stdin = stdin.lock();
    let mut buf = [0; 1];
    // read one byte from stdin
    stdin.read(&mut buf).unwrap();
    // push the ascii value to the stack
    int_stack.push(buf[0] as i64);
    return 0;
}

// return true if the word is in the dictionary
fn is_in_compiled_words(word: String, compiled_words: &Vec<Vec<String>>) -> bool {
    for i in 0..compiled_words.len() {
        if word == compiled_words[i][0] {
            return true;
        }
    }
    return false;
}

// return the  word in the compiled words f existing
fn get_word(word: String, compiled_words: &Vec<Vec<String>>) -> Vec<String> {
    for i in 0..compiled_words.len() {
        if word == compiled_words[i][0] {
            return compiled_words[i].clone();
        }
    }
    return Vec::new();
}

// return the index of the word in the compiled words
fn get_index(word: String, compiled_words: &Vec<Vec<String>>) -> i32 {
    for i in 0..compiled_words.len() {
        if word == compiled_words[i][0] {
            return i as i32;
        }
    }
    return -1;
}

fn execute_words(input: String, int_stack: &mut Vec<i64>, compiled_words: &mut Vec<Vec<String>>, compile_flag: &mut bool) -> i32 {
    let input: &str = &input;
    match input {
        // builtin dictionnary
        ".s" => { // show length and content of the stack
            print!("<{}> ", int_stack.len());
            for e in int_stack {
                print!("{} ", e);
            }
        }
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
        "drop" => { // remove the top number of the stack
            if drop(int_stack) == -1 {
                return -1
            }
        }
        "cr" => { // carriage return
            println!("");
        }
        ":" => { // set compile mode
            *compile_flag = true;
            compiled_words.push(Vec::new());
        }
        ";" => {
            println!("not in compile mode");
            return -1;
        }

        // _TODO_ adding more builtin words here

        "key" => {
            key(int_stack);
        }
        "words" => {
            // show compiled words
            for i in 0..compiled_words.len() {
                println!("{:?}", compiled_words[i]);
            }
        }
        "bye" => return 1,
        _ => { // else check if it's in compiled words
            if is_in_compiled_words(input.to_string(), compiled_words) {
                let word = get_word(input.to_string(), compiled_words);
                // proccess the word
                let word = word[1..].to_vec(); // remove the word from the compiled words to avoid infinite loop
                process_input(word, int_stack, compile_flag, compiled_words);
            } else {
                println!("{} : not known", input);
                return -1;
            }
        }
    }
    return 0;
}

// process each words given by a file or a user
fn process_input(input_array: Vec<String>, int_stack: &mut Vec<i64>, compile_flag: &mut bool, compiled_words: &mut Vec<Vec<String>>) -> i32 {
    for input in input_array {
        if *compile_flag { // enter in compile mode
            if input == ";" { // exit compile mode
                *compile_flag = false;
                // if there is at least one word in the compiled words
                // check if the word already exist
                if compiled_words.len() > 0 {
                    let last_word = compiled_words[compiled_words.len()-1].clone();
                    compiled_words.pop();
                    if is_in_compiled_words(last_word[0].clone(), compiled_words) {
                        // remove the word from the compiled words
                        let index = get_index(last_word[0].clone(), compiled_words);
                        compiled_words.remove(index as usize);
                        // add the new word to the compiled words
                        compiled_words.push(last_word.clone());
                        println!("{} : redefined", last_word[0]);
                    } else {
                        // add the new word to the compiled words
                        compiled_words.push(last_word.clone());
                    }
                }
            } else { // add the word to the compiled words
                let len = compiled_words.len();
                compiled_words[len-1].push(input.to_string());
            }
        } else {
            match input.parse::<i64>() {
                Ok(n) => { // if number then push it to the stack
                    int_stack.push(n)
                }
                Err(_) => { // else check dictionnary
                    match execute_words(input, int_stack, compiled_words, compile_flag) {
                        -1 => return -1, // word not found
                        1 => return 1, // bye
                        _ => {} // else
                    }
                }
            }
        } 
    }
    return 0;
}

fn get_input() -> Vec<String> {
    let fd = stdin();
    let mut line = String::new();
    fd.read_line(&mut line)
        .expect("err: read_line()");
    line = line.trim().to_string();
    let mut input_array: Vec<String> = line.split_whitespace().map(str::to_string).collect();
    // remove all empty element
    input_array.retain(|s| *s != *"");
    return input_array;
}

// taking input from the user
fn stdin_interpreter() {
    let mut int_stack: Vec<i64> = Vec::new(); // stack of number
    let mut compile_flag: bool = false; // flag to enter in compile mode
    let mut compiled_words: Vec<Vec<String>> = Vec::new(); // contain array of compiled words
    println!("Welcome to rForth\ntype 'bye' to exit");
    loop {
        let input_array = get_input();
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
    }
}

// reading from a file
fn file_interpreter(f: File) {
    let mut fd = BufReader::new(f);
    let mut int_stack: Vec<i64> = Vec::new();
    let mut line = String::new();
    let mut compile_flag: bool = false;
    let mut compiled_words: Vec<Vec<String>> = Vec::new();
    while fd.read_line(&mut line).unwrap() > 0 {
        line = line.trim().to_string();
        let input_array = line.split_whitespace().map(str::to_string).collect();
        match process_input(input_array, &mut int_stack, &mut compile_flag, &mut compiled_words) {
            1 => return, // bye
            -1 => (), // word not found
            _ => (),
        }
        line.clear();
    }
    stdin_interpreter();
}

fn main() {
    // check argument to either read from a file or using stdin
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            stdin_interpreter()
        }
        2 => {
            let f = match File::open(&args[1]) {
                Ok(file) => file,
                Err(why) => {
                    println!("err: {}", why);
                    stdin_interpreter();
                    return			
                }
            };
            file_interpreter(f)
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

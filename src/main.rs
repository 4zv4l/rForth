use std::io::{stdin, BufReader, BufRead};
use std::fs::File;
use strs::{Stack, CompiledWords};

mod words;
mod compile;
mod strs;

// execute one word
fn execute_words(input: String, int_stack: &mut Stack<i64>, compiled_words: &mut CompiledWords, compile_flag: &mut bool) -> i32 {
    let input: &str = &input;
    match input {
        // builtin dictionnary
        ".s" => { // show length and content of the stack
            print!("<{}> ", int_stack.len());
            for i in 0..int_stack.len() {
                print!("{} ", int_stack.inner[i]);
            }
        }
        "show" => {
            print!("{}", int_stack);
        }
        "+" => {
            if words::add(int_stack) == -1 {
                return -1
            }
        }
        "-" => {
            if words::sub(int_stack) == -1 {
                return -1
            }
        }
        "*" => {
            if words::mul(int_stack) == -1 {
                return -1
            }
        }
        "/" => {
            if words::div(int_stack) == -1 {
                return -1
            }
        }
        "drop" => { // remove the top number of the stack
            if words::drop(int_stack) == -1 {
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
        r#".""# => {
            println!("start string");
        }

        // _TODO_ adding more builtin words here

        "key" => {
            words::key(int_stack);
        }
        "emit" => { // print the top number of the stack in ascii
            words::emit(int_stack);
        }
        "words" => {
            // show compiled words
            for i in 0..compiled_words.len() {
                println!("{:?}", compiled_words[i]);
            }
        }
        "bye" => return 1,
        _ => { // else check if it's in compiled words
            if compile::is_in_compiled_words(input.to_string(), compiled_words) {
                let word = compile::get_word(input.to_string(), compiled_words);
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
fn process_input(input_array: Vec<String>, int_stack: &mut Stack<i64>, compile_flag: &mut bool, compiled_words: &mut CompiledWords) -> i32 {
    for input in input_array {
        if *compile_flag { // enter in compile mode
            if input == ";" { // exit compile mode
                *compile_flag = false;
                // if there is at least one word in the compiled words
                // check if the word already exist
                if compiled_words.len() > 0 {
                    let last_word = compiled_words[compiled_words.len()-1].clone();
                    compiled_words.pop();
                    if compile::is_in_compiled_words(last_word[0].clone(), compiled_words) {
                        // remove the word from the compiled words
                        let index = compile::get_index(last_word[0].clone(), compiled_words);
                        compiled_words.remove(index);
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

// get input from the user until ENTER and return each word entered
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
    let mut int_stack: Stack<i64> = Stack::new(); // stack of number
    let mut compile_flag: bool = false; // flag to enter in compile mode
    let mut compiled_words: CompiledWords = CompiledWords::new(); // contain array of compiled words
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
    let mut int_stack: Stack<i64> = Stack::new(); // stack of number
    let mut line = String::new();
    let mut compile_flag: bool = false;
    let mut compiled_words: CompiledWords = CompiledWords::new();
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
	// if file doesn't end with 'bye' enter into stdin interpreter mode
    stdin_interpreter();
}

fn main() {
    // check argument to either read from a file or stdin
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

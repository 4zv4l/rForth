use crossterm;
use crossterm::event::{read, Event, KeyCode};

use crate::{Stack};

// take 2 top numbers from the stack and multiply them
pub fn mul(int_stack: &mut Stack<i64>) -> i32 {
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

// take 2 top numbers from the stack and divide them
// take care of 0 division
pub fn div(int_stack: &mut Stack<i64>) -> i32 {
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
pub fn sub(int_stack: &mut Stack<i64>) -> i32 {
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
    int_stack.push(n2-n1);
    return 0
}

// take 2 top numbers from the stack and addition them
pub fn add(int_stack: &mut Stack<i64>) -> i32 {
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
pub fn drop(int_stack: &mut Stack<i64>) -> i32 {
    if int_stack.is_empty() {
        println!("Stack underflow");
        return -1
    }
    int_stack.pop();
    return 0;
}

// print the top number of the stack
pub fn dot(int_stack: &mut Stack<i64>) -> i32 {
    if int_stack.is_empty() {
        println!("Stack underflow");
        return -1
    }
    let n = match int_stack.pop() {
        None =>  {
            println!("Stack underflow");
            return -1
        }
        Some(n) => n,
    };
    println!("{}", n);
    return 0;
}

// modulo of the top two numbers of the stack
pub fn modulo(int_stack: &mut Stack<i64>) -> i32 {
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
    int_stack.push(n2%n1);
    return 0
}

// duplicate the top number of the stack
pub fn dup(int_stack: &mut Stack<i64>) -> i32 {
    if int_stack.is_empty() {
        println!("Stack underflow");
        return -1
    }
    let n = match int_stack.pop() {
        None =>  {
            println!("Stack underflow");
            return -1
        }
        Some(n) => n,
    };
    int_stack.push(n);
    int_stack.push(n);
    return 0
}

// print the top number of the stack in ascii
pub fn emit(int_stack: &mut Stack<i64>) -> i32 {
    match int_stack.pop() {
        None =>  {
            println!("Stack underflow");
            return -1
        }
        Some(n) => print!("{}", n as u8 as char),
    };
    return 0;
}

// get one key from the user and push the ascii value to the stack
pub fn key(int_stack: &mut Stack<i64>) {
    // set terminal in raw mode
    crossterm::terminal::enable_raw_mode().unwrap();
    loop {
        // `read` until the user presses a key
        if let Event::Key(key_event) = read().unwrap() {
            // If the key event is triggered by the user entering a key and not
            // other keys like enter, delete and arrow keys.
            if let KeyCode::Char(c) = key_event.code {
                // push the ascii value to the stack
                int_stack.push(c as i64);
                break;
            }
            // continue with other keys
        }
    }
    // reset terminal
    crossterm::terminal::disable_raw_mode().unwrap();
}


// basic jump
// execute the next instruction anyway
/*
pub fn branch(int_stack: &mut Stack<i64>, pc: &mut usize) -> i32 {
    let n = match int_stack.pop() {
        None =>  {
            println!("Stack underflow");
            return -1
        }
        Some(n) => n,
    };
    *pc = n as usize;
    return 0
}

// conditional jump
// execute the next instruction if the top number is not zero
pub fn branch_if(int_stack: &mut Stack<i64>, pc: &mut usize) -> i32 {
    let n = match int_stack.pop() {
        None =>  {
            println!("Stack underflow");
            return -1
        }
        Some(n) => n,
    };
    if n != 0 {
        *pc = n as usize;
    }
    return 0
}
*/

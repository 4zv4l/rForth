// take 2 top numbers from the stack and multiply them
pub fn mul(int_stack: &mut Vec<i64>) -> i32 {
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
pub fn div(int_stack: &mut Vec<i64>) -> i32 {
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
pub fn sub(int_stack: &mut Vec<i64>) -> i32 {
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
pub fn add(int_stack: &mut Vec<i64>) -> i32 {
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
pub fn drop(int_stack: &mut Vec<i64>) -> i32 {
    if int_stack.is_empty() {
        println!("Stack underflow");
        return -1
    }
    int_stack.pop();
    return 0;
}

// get one key from the user and push the ascii value to the stack
pub fn key(int_stack: &mut Vec<i64>) {
    // idk how to do this in rust
}

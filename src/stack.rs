use std::fmt;

pub struct Stack<T> {
    pub inner: Vec<T>
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            inner: Vec::new()
        }
    }

    pub fn push(&mut self, value: T) {
        self.inner.push(value);
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl fmt::Display for Stack<i64> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.inner.len() {
            write!(f, "{} ", self.inner[i])?;
        }
        Ok(())
    }
}
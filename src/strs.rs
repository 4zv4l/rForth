use std::fmt;
use std::ops;

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

pub struct CompiledWords {
    pub inner: Vec<Vec<String>>
}

impl CompiledWords {
    pub fn new() -> CompiledWords {
        CompiledWords {
            inner: Vec::new()
        }
    }

    pub fn push(&mut self, value: Vec<String>) {
        self.inner.push(value);
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn pop(&mut self) -> Option<Vec<String>> {
        self.inner.pop()
    }

    pub fn remove(&mut self, index: i32) {
        self.inner.remove(index as usize);
    }

}

// implement indexing for CompiledWords
impl ops::Index<usize> for CompiledWords {
    type Output = Vec<String>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

// implement IndexMut for CompiledWords
impl ops::IndexMut<usize> for CompiledWords {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}
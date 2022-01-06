use std::fmt;
use std::ops;
use std::collections::HashMap;

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

// Heap implementation to store data using 8bits address and can store up to 16bits data
pub struct Heap {
    pub inner: HashMap<u8, i64> 
    // address u8 from 0 to 255
}

impl Heap {
    pub fn new() -> Heap {
        Heap {
            inner: HashMap::new()
        }
    }

    /*
    // alloc a new chunk of memory in random address
    pub fn alloc(&mut self) -> u8 {
        // find not used memory address on the heap
        let mut address = 0;
        while self.inner.contains_key(&address) {
            address += 1;
        }
        // allocate the chunk of memory
        self.inner.insert(address, 0);
        return address;
    }

    pub fn free(&mut self, address: u8) {
        self.inner.remove(&address);
    }
    */

    pub fn fetch(&self, address: u8) -> Option<i64> {
        match self.inner.get(&address) {
            Some(value) => Some(*value),
            None => None
        }
    }

    pub fn store(&mut self, address: u8, value: i64) {
        self.inner.insert(address, value);
    }
}

// show the heap ordered by address
impl fmt::Display for Heap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sorted: Vec<_> = self.inner.iter().collect();
        sorted.sort_by(|a, b| a.0.cmp(b.0));
        for i in 0..sorted.len() {
            write!(f, "addr {:<3} : {}\n", sorted[i].0, sorted[i].1)?;
        }
        Ok(())
    }
}

/*

find a way to make variable 

pub struct VariableList {
    pub inner: Vec<Variable>
}

impl VariableList {
    pub fn new() -> VariableList {
        VariableList {
            inner: Vec::new()
        }
    }

    pub fn push(&mut self, value: Variable) {
        self.inner.push(value);
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn pop(&mut self) -> Option<Variable> {
        self.inner.pop()
    }

    pub fn remove(&mut self, index: i32) {
        self.inner.remove(index as usize);
    }
}

pub struct Variable {
    name: String,
    value: u8 // address
}

impl Variable {
    fn new(name: String, value: u8) -> Variable {
        Variable {
            name: name,
            value: value
        }
    }
}
*/
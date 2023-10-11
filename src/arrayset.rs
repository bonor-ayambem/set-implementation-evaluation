use std::vec::Vec;

pub struct Arrayset {
    set: Vec<i32>,
}

impl Arrayset {
    pub fn new() -> Self {
        Arrayset {
            set: Vec::new(),
        }
    }
    pub fn insert(&mut self, num: i32) -> bool {
        if self.find(num) {
            return false;
        }
        else {
            self.set.insert(self.set.len(), num);
        }
        true
    }
    pub fn remove(&mut self, num: i32) -> bool {
        for i in 0..self.set.len() {
            if self.set[i] == num {
                self.set.remove(i);
                return true;
            }
        }
        false
        // removes by index so have to iterate through
    }
    pub fn find(&self, num: i32) -> bool {
        for i in 0..self.set.len() {
            if self.set[i] == num {
                return true;
            }
        }
        
        false
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }
}
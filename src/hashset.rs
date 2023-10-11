use std::collections::HashSet;

pub struct Hashset {
    set: HashSet<i32>,
}

impl Hashset {
    pub fn new() -> Self {
        Hashset {
            set: HashSet::new(),
        }
    }
    pub fn insert(&mut self, num: i32) -> bool {
        self.set.insert(num)
    }
    pub fn remove(&mut self, num: i32) -> bool {
        self.set.remove(&num)
    }
    pub fn find(&self, num: i32) -> bool {
        self.set.contains(&num)
    }
    pub fn len(&self) -> usize {
        self.set.len()
    }
}
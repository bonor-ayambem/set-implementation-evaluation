use std::collections::BTreeMap;

pub struct Treeset {
    set: BTreeMap<i32, i32>, // can make value empty
    count: i32,
}

impl Treeset {
    pub fn new() -> Self {
        Treeset {
            set: BTreeMap::new(),
            count: 0,
        }
    }
    pub fn insert(&mut self, num: i32) -> bool { 
        let operation: Option<i32> = self.set.insert(num, self.count);
        self.count += 1;
        match operation {
            None => true,
            _ => false,
        }
        // returns Option type which must handle error
        // also takes in a key-value pair?
        // ordered in ascending order of the keys. how does this affect removing and finding

        // the insert function for btree returns none if key is present. counter-intuitive to 
        //  how i want mine implemented
    }
    pub fn remove(&mut self, num: i32) -> bool {
        let operation: Option<i32> = self.set.remove(&num);
        match operation {
            None => false,
            _ => true,
        }
    }
    pub fn find(&self, num: i32) -> bool {
        self.set.contains_key(&num)
    }
    pub fn len(&self) -> usize {
        self.set.len()
    }
}
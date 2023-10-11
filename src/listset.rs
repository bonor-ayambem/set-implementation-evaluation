use std::collections::LinkedList;

pub struct Listset {
    set: LinkedList<i32>,
}

impl Listset {
    pub fn new() -> Self {
        Listset {
            set: LinkedList::new(),
        }
    }
    pub fn insert(&mut self, num: i32) -> bool { // only have access to first and last elements... 
        if self.set.contains(&num) {
            false
        }
        else {
            self.set.push_back(num);
            true
        }

    }
    pub fn remove(&mut self, num: i32) -> bool { 
        let mut iter = self.set.iter();

        for i in 0..self.set.len() {
            let element: Option<&i32> = iter.next();
            match element {
                Some(val) if *val == num => {
                    self.set.remove(i);
                    return true;
                },
                _ => (),
            }
        }

        false
        // using remove() required addition of crate attribute ![#feature...] to the root module, and instally nightly

        // not retain method and remove method removes based on index
        // have to use and iterator and keep track of indices as we move
    }
    pub fn find(&self, num: i32) -> bool {
        self.set.contains(&num)
    }
    pub fn len(&self) -> usize { // had to create this method
        self.set.len()
    }
}
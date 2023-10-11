#![feature(test)]
use rand::Rng;

extern crate test;
use test::Bencher;

use homework_1::hashset::Hashset;
use homework_1::treeset::Treeset;
use homework_1::listset::Listset;
use homework_1::arrayset::Arrayset;

/******
//
// HASHSET BENCHMARKING FUNCTIONS ARE BELOW
//
******/

#[bench]
fn hashset_1k_0(b: &mut Bencher) {
    let r = 0;
    let size = 1_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Hashset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn hashset_1k_20(b: &mut Bencher) {
    let r = 20;
    let size = 1_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Hashset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 { // had to create len() methods to achieve this
        let num: i32 = rng.gen_range(0..i32::MAX); // doesn't matter? could this be 0..1_000? is there anything lost to a smaller range?
        set.insert(num);
    }

    b.iter(|| { // this is the only part that should be in here? v
        // let mut num_finds = 0;
        // let mut num_inserts = 0;
        // let mut num_removes = 0;

        for _ in 0..i { // i (number of ops) is fixed as 100_000
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
                // num_finds += 1;
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
                // num_inserts += 1;
            }
            else {
                set.remove(key);
                // num_removes += 1;
            }
        }
    });
}

#[bench]
fn hashset_1k_50(b: &mut Bencher) {
    let r = 50;
    let size = 1_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Hashset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn hashset_1k_80(b: &mut Bencher) {
    let r = 80;
    let size = 1_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Hashset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn hashset_1k_100(b: &mut Bencher) {
    let r = 100;
    let size = 1_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Hashset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn hashset_10k_0(b: &mut Bencher) {
    let r = 0;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Hashset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn hashset_10k_20(b: &mut Bencher) {
    let r = 20;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Hashset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 { // had to create len() methods to achieve this
        let num: i32 = rng.gen_range(0..i32::MAX); // doesn't matter? could this be 0..1_000? is there anything lost to a smaller range?
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn hashset_10k_50(b: &mut Bencher) {
    let r = 50;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Hashset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn hashset_10k_80(b: &mut Bencher) {
    let r = 80;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Hashset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn hashset_10k_100(b: &mut Bencher) {
    let r = 100;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Hashset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn hashset_100k_0(b: &mut Bencher) {
    let r = 0;
    let size = 100_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Hashset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn hashset_100k_20(b: &mut Bencher) {
    let r = 20;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Hashset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 { // had to create len() methods to achieve this
        let num: i32 = rng.gen_range(0..i32::MAX); // doesn't matter? could this be 0..1_000? is there anything lost to a smaller range?
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn hashset_100k_50(b: &mut Bencher) {
    let r = 50;
    let size = 100_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Hashset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn hashset_100k_80(b: &mut Bencher) {
    let r = 80;
    let size = 100_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Hashset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn hashset_100k_100(b: &mut Bencher) {
    let r = 100;
    let size = 100_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Hashset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn hashset_1m_0(b: &mut Bencher) {
    let r = 0;
    let size = 1_000_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Hashset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn hashset_1m_20(b: &mut Bencher) {
    let r = 20;
    let size = 1_000_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Hashset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 { // had to create len() methods to achieve this
        let num: i32 = rng.gen_range(0..i32::MAX); // doesn't matter? could this be 0..1_000? is there anything lost to a smaller range?
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn hashset_1m_50(b: &mut Bencher) {
    let r = 50;
    let size = 1_000_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Hashset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn hashset_1m_80(b: &mut Bencher) {
    let r = 80;
    let size = 1_000_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Hashset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn hashset_1m_100(b: &mut Bencher) {
    let r = 100;
    let size = 1_000_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Hashset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

/******
//
// TREESET BENCHMARKING FUNCTIONS ARE BELOW
//
******/

#[bench]
fn treeset_1k_0(b: &mut Bencher) {
    let r = 0;
    let size = 1_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Treeset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn treeset_1k_20(b: &mut Bencher) {
    let r = 20;
    let size = 1_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Treeset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 { // had to create len() methods to achieve this
        let num: i32 = rng.gen_range(0..i32::MAX); // doesn't matter? could this be 0..1_000? is there anything lost to a smaller range?
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn treeset_1k_50(b: &mut Bencher) {
    let r = 50;
    let size = 1_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Treeset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn treeset_1k_80(b: &mut Bencher) {
    let r = 80;
    let size = 1_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Treeset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn treeset_1k_100(b: &mut Bencher) {
    let r = 100;
    let size = 1_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Treeset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn treeset_10k_0(b: &mut Bencher) {
    let r = 0;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Treeset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn treeset_10k_20(b: &mut Bencher) {
    let r = 20;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Treeset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 { // had to create len() methods to achieve this
        let num: i32 = rng.gen_range(0..i32::MAX); // doesn't matter? could this be 0..1_000? is there anything lost to a smaller range?
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn treeset_10k_50(b: &mut Bencher) {
    let r = 50;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Treeset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn treeset_10k_80(b: &mut Bencher) {
    let r = 80;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Treeset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn treeset_10k_100(b: &mut Bencher) {
    let r = 100;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Treeset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn treeset_100k_0(b: &mut Bencher) {
    let r = 0;
    let size = 100_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Treeset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn treeset_100k_20(b: &mut Bencher) {
    let r = 20;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Treeset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 { // had to create len() methods to achieve this
        let num: i32 = rng.gen_range(0..i32::MAX); // doesn't matter? could this be 0..1_000? is there anything lost to a smaller range?
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn treeset_100k_50(b: &mut Bencher) {
    let r = 50;
    let size = 100_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Treeset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn treeset_100k_80(b: &mut Bencher) {
    let r = 80;
    let size = 100_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Treeset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn treeset_100k_100(b: &mut Bencher) {
    let r = 100;
    let size = 100_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Treeset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn treeset_1m_0(b: &mut Bencher) {
    let r = 0;
    let size = 1_000_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Treeset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn treeset_1m_20(b: &mut Bencher) {
    let r = 20;
    let size = 1_000_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Treeset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 { // had to create len() methods to achieve this
        let num: i32 = rng.gen_range(0..i32::MAX); // doesn't matter? could this be 0..1_000? is there anything lost to a smaller range?
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn treeset_1m_50(b: &mut Bencher) {
    let r = 50;
    let size = 1_000_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Treeset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn treeset_1m_80(b: &mut Bencher) {
    let r = 80;
    let size = 1_000_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Treeset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn treeset_1m_100(b: &mut Bencher) {
    let r = 100;
    let size = 1_000_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Treeset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

/******
//
// LISTSET BENCHMARKING FUNCTIONS ARE BELOW
//
******/

#[bench]
fn listset_1k_0(b: &mut Bencher) {
    let r = 0;
    let size = 1_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Listset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn listset_1k_20(b: &mut Bencher) {
    let r = 20;
    let size = 1_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Listset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 { // had to create len() methods to achieve this
        let num: i32 = rng.gen_range(0..i32::MAX); // doesn't matter? could this be 0..1_000? is there anything lost to a smaller range?
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn listset_1k_50(b: &mut Bencher) {
    let r = 50;
    let size = 1_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Listset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn listset_1k_80(b: &mut Bencher) {
    let r = 80;
    let size = 1_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Listset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn listset_1k_100(b: &mut Bencher) {
    let r = 100;
    let size = 1_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Listset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn listset_10k_0(b: &mut Bencher) {
    let r = 0;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Listset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn listset_10k_20(b: &mut Bencher) {
    let r = 20;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Listset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 { // had to create len() methods to achieve this
        let num: i32 = rng.gen_range(0..i32::MAX); // doesn't matter? could this be 0..1_000? is there anything lost to a smaller range?
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn listset_10k_50(b: &mut Bencher) {
    let r = 50;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Listset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn listset_10k_80(b: &mut Bencher) {
    let r = 80;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Listset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn listset_10k_100(b: &mut Bencher) {
    let r = 100;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Listset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn listset_100k_0(b: &mut Bencher) {
    let r = 0;
    let size = 100_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Listset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn listset_100k_20(b: &mut Bencher) {
    let r = 20;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Listset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 { // had to create len() methods to achieve this
        let num: i32 = rng.gen_range(0..i32::MAX); // doesn't matter? could this be 0..1_000? is there anything lost to a smaller range?
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn listset_100k_50(b: &mut Bencher) {
    let r = 50;
    let size = 100_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Listset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn listset_100k_80(b: &mut Bencher) {
    let r = 80;
    let size = 100_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Listset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn listset_100k_100(b: &mut Bencher) {
    let r = 100;
    let size = 100_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Listset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn listset_1m_0(b: &mut Bencher) {
    let r = 0;
    let size = 1_000_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Listset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn listset_1m_20(b: &mut Bencher) {
    let r = 20;
    let size = 1_000_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Listset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 { // had to create len() methods to achieve this
        let num: i32 = rng.gen_range(0..i32::MAX); // doesn't matter? could this be 0..1_000? is there anything lost to a smaller range?
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn listset_1m_50(b: &mut Bencher) {
    let r = 50;
    let size = 1_000_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Listset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn listset_1m_80(b: &mut Bencher) {
    let r = 80;
    let size = 1_000_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Listset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn listset_1m_100(b: &mut Bencher) {
    let r = 100;
    let size = 1_000_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Listset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

/******
//
// ARRAYSET BENCHMARKING FUNCTIONS ARE BELOW
//
******/

#[bench]
fn arrayset_1k_0(b: &mut Bencher) {
    let r = 0;
    let size = 1_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Arrayset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn arrayset_1k_20(b: &mut Bencher) {
    let r = 20;
    let size = 1_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Arrayset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 { // had to create len() methods to achieve this
        let num: i32 = rng.gen_range(0..i32::MAX); // doesn't matter? could this be 0..1_000? is there anything lost to a smaller range?
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn arrayset_1k_50(b: &mut Bencher) {
    let r = 50;
    let size = 1_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Arrayset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn arrayset_1k_80(b: &mut Bencher) {
    let r = 80;
    let size = 1_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Arrayset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn arrayset_1k_100(b: &mut Bencher) {
    let r = 100;
    let size = 1_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Arrayset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn arrayset_10k_0(b: &mut Bencher) {
    let r = 0;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Arrayset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn arrayset_10k_20(b: &mut Bencher) {
    let r = 20;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Arrayset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 { // had to create len() methods to achieve this
        let num: i32 = rng.gen_range(0..i32::MAX); // doesn't matter? could this be 0..1_000? is there anything lost to a smaller range?
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn arrayset_10k_50(b: &mut Bencher) {
    let r = 50;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Arrayset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn arrayset_10k_80(b: &mut Bencher) {
    let r = 80;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Arrayset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn arrayset_10k_100(b: &mut Bencher) {
    let r = 100;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Arrayset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn arrayset_100k_0(b: &mut Bencher) {
    let r = 0;
    let size = 100_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Arrayset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn arrayset_100k_20(b: &mut Bencher) {
    let r = 20;
    let size = 10_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Arrayset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 { // had to create len() methods to achieve this
        let num: i32 = rng.gen_range(0..i32::MAX); // doesn't matter? could this be 0..1_000? is there anything lost to a smaller range?
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn arrayset_100k_50(b: &mut Bencher) {
    let r = 50;
    let size = 100_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Arrayset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn arrayset_100k_80(b: &mut Bencher) {
    let r = 80;
    let size = 100_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Arrayset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn arrayset_100k_100(b: &mut Bencher) {
    let r = 100;
    let size = 100_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Arrayset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn arrayset_1m_0(b: &mut Bencher) {
    let r = 0;
    let size = 1_000_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Arrayset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn arrayset_1m_20(b: &mut Bencher) {
    let r = 20;
    let size = 1_000_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Arrayset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 { // had to create len() methods to achieve this
        let num: i32 = rng.gen_range(0..i32::MAX); // doesn't matter? could this be 0..1_000? is there anything lost to a smaller range?
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn arrayset_1m_50(b: &mut Bencher) {
    let r = 50;
    let size = 1_000_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Arrayset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn arrayset_1m_80(b: &mut Bencher) {
    let r = 80;
    let size = 1_000_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Arrayset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

#[bench]
fn arrayset_1m_100(b: &mut Bencher) {
    let r = 100;
    let size = 1_000_000;
    let i = 1_000;
    //k is always max

    let ratio = r/100 * i;
    
    let mut set = Arrayset::new();

    let mut rng = rand::thread_rng();
    while set.len() < size/2 {
        let num: i32 = rng.gen_range(0..i32::MAX);
        set.insert(num);
    }

    b.iter(|| {
        for _ in 0..i {
            let op_type: usize = rng.gen_range(0..i);
            let key: i32 = rng.gen_range(0..i32::MAX);

            if op_type < ratio {
                set.find(key);
            }
            else if op_type < (i + (i-ratio)/2) && set.len() < size {
                set.insert(key);
            }
            else {
                set.remove(key);
            }
        }
    });
}

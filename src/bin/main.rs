// use homework_1::hashset::Hashset;
// use homework_1::treeset::Treeset;
// use homework_1::listset::Listset;
// use homework_1::arrayset::Arrayset;

use std::io;

fn main() {
    println!("Please provide input in the following format: \"i k d r\"
    i (int) - number of operations to be performed on your set
    k (int) - max value to be inserted into the set
    d (string) - one of the following: \"array\", \"list\", \"tree\", \"hashtable\"
    r (int) - percentage of find operations, the remaining will be equally split between insert and remove");

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    // confirm that correct input is entered?

    println!("input accepted!");    
}

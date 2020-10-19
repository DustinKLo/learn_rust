#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

// testing out functions with vector as input
fn print_vec(input_vec: &Vec<i32>) {
    for n in input_vec {
        print!("{} ", n);
    }
    println!("");
}

// testing out how to increment array through a function
fn increment_vec(input_vec: &mut Vec<i32>, incr: i32) {
    for n in input_vec {
        *n += incr;
    }
    println!("");
}

fn main() {
    let nums: Vec<i32> = Vec::new(); // need to specify type in vector
    let mut nums = vec![1, 2, 3, 4, 5]; // more concise way to create a vector
    nums.push(6);
    nums.push(7);
    nums.push(8);

    // accessing speicfic elements in vector
    &nums[2];
    &nums[2..];

    nums.remove(2);

    // iterating over vector
    for num in &nums {
        print!("{} ", num);
    }
    println!("");
    print_vec(&nums);

    // modifying values in place
    for num in &mut nums {
        *num += 1;
    }
    print_vec(&nums);
    increment_vec(&mut nums, 5);
    print_vec(&nums);

    // random types in vectors
    enum Value {
        Int(i32),
        Float(f32),
    };
    let random = vec![Value::Int(3), Value::Float(3.3)];

    // ##########################
    // hash map
    let mut numbers: HashMap<&str, i32> = HashMap::new();

    // insert/update values
    numbers.insert("one", 1);
    numbers.insert("two", 2);
    numbers.insert("three", 3);
    println!("{:?}", numbers); // how to print hashmap

    // access values
    // if key doesn't exist the .unwrap() will raise an error
    // get returns a Options type (Some or None)
    println!("{}", numbers.get("two").unwrap());

    // checking if key exists in HashMap
    if numbers.contains_key("two") {
        // checking if key exists
        // similar to python, "key" in dict()
        println!("key exists: two");
    }

    match numbers.get("nineteen") {
        Some(val) => println!("{}", val),
        None => println!("key doesn't exist"),
    }

    // remove hashmap value
    numbers.remove("three");

    // iterate over hashmap (NOT ORDERED)
    for (key, value) in &numbers {
        println!("{} => {}", key, value);
    }
}

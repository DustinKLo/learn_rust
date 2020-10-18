#[allow(dead_code, unused_variables)]

fn main() {
    let age = 25;

    if age > 21 {
        println!("over 21");
    } else if age < 21 {
        println!("under 21");
    } else {
        println!("exactly 21");
    }

    // if with output capture (rust's version of terniary operator)
    let old_enough = if age > 21 { true } else { false };

    // match statement (used like a switch)
    match age {
        21 => println!("age is 21"),
        22 => println!("age is 22"),
        23 | 24 => println!("age is 23 or 24"),
        25..=28 => println!("age is between 25 and 28"),
        n if n < 5 => println!("age is less than 5"),
        n if n > 50 => println!("age is greater than 50"),
        _ => println!("age is something else"),
    }

    // infinite loop
    let mut i = 0;
    println!("");
    let x = loop {
        // setting x to ending value of loop
        println!("i: {}", i);
        if i == 10 {
            break i; // returning i (10) into x
        }
        i += 1;
    };
    println!("x is now {}\n", x);

    // while loop
    let mut j = 0;
    while j < 10 {
        println!("j: {}", j);
        j += 1;
    }
    println!("");

    // for loop
    for i in 0..11 {
        println!("i: {}", i);
    }

    println!("\nwith .step_by(n)");
    for i in (0..21).step_by(2) {
        // step_by(n) allows you to jump by n every loop
        println!("i: {}", i);
    }
    println!("");

    // for loop over array (iteration)
    let nums = vec![1, 2, 3, 4, 5, 6, 6, 7, 8, 9];
    for num in nums {
        println!("num: {}", num);
    }
}

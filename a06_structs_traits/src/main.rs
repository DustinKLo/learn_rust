#![allow(dead_code, unused_variables)]

// structs
struct Data {
    num1: i32,
    num2: i32,
    str1: String,
    optional_num: Option<i32>,
}

struct TwoNums(i32, i32); // tuple struct

// ###################################
// implement method to struct
impl Data {
    // setting defaults (kind of like a constructor)
    fn new() -> Self {
        Data {
            num1: 2,
            num2: 4,
            str1: "placeholder".to_string(),
            optional_num: Some(8),
        }
    }
    fn sum(&self) -> i32 {
        return self.num1 + self.num2;
    }
}
// can split methods into multiple "impl"
impl Data {
    fn multiply(&self) -> i32 {
        return self.num1 * self.num2;
    }
}

// unit struct
struct Calculator;

impl Calculator {
    fn add(n1: i32, n2: i32) -> i32 {
        return n1 + n2;
    }
    fn subtract(n1: i32, n2: i32) -> i32 {
        return n1 - n2;
    }
    fn multiply(n1: i32, n2: i32) -> i32 {
        return n1 * n2;
    }
    fn divide(n1: i32, n2: i32) -> i32 {
        return n1 / n2;
    }
}

// #############################################
// TRAITS, skeleton (series of methods) that can be used to create a struct
// up to developer to implement the methods themselves
trait Transform {
    fn reverse(&self) -> String;

    fn output_reverse(&self) {
        println!("{}", self.reverse())
    }

    fn example_trait_func(&self) {
        println!("abba dabba doo i'm a method in a trait");
    }
}

impl Transform for Data {
    fn reverse(&self) -> String {
        return self.str1.chars().rev().collect::<String>();
    }
}

fn main() {
    let a = Data {
        num1: 4,
        num2: 3,
        str1: "Dustin".to_string(),
        optional_num: None,
    };
    println!("a sum: {}", a.sum());

    let b = Data { num1: 8, ..a }; // using the rest of the properties (except num1) from a

    let mut c = Data::new();
    println!("c.num1: {} + {} = {}", c.num1, c.num2, c.sum());
    c.num1 = 100;
    println!("c.num1: {} + {} = {}", c.num1, c.num2, c.sum());
    println!("c.num1: {} * {} = {}", c.num1, c.num2, c.multiply());
    println!("{} {}", c.str1, c.reverse());
    c.output_reverse();
    c.example_trait_func();

    let d = TwoNums(32, 33);
    println!("TwoNums d: {} {}", d.0, d.1);

    println!("{}", Calculator::add(3, 4));
    println!("{}", Calculator::subtract(3, 4));
    println!("{}", Calculator::multiply(3, 4));
    println!("{}", Calculator::divide(3, 4));
}

#![allow(dead_code, unused_variables)]

use std::io::{stdin, stdout, Write};

// read() helps read inputs from the terminal
fn read(input: &mut String) {
    stdout().flush().expect("failed to flush"); // flush everything from stdout
    stdin().read_line(input).expect("failed to read");
}

fn main() {
    println!("Calculator app!");
    println!("----------------");

    loop {
        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut operator = String::new();

        print!("What is the first number? ");
        read(&mut num1); // read from terminal and set value as num1

        print!("What is the second number? ");
        read(&mut num2); // read from terminal and set value as num2

        print!("What is the operator? [+ - * /] ");
        read(&mut operator); // read from terminal and set value as operator

        // parse string to int
        // because terminal inputs have a trailing \n at the end, need to trim the string
        let num1: f32 = num1.trim().parse().unwrap();
        let num2: f32 = num2.trim().parse().unwrap();

        let operator: char = operator.trim().chars().next().unwrap(); // parse string to character

        let operators = String::from("+-*/");
        if !operators.contains(operator) {
            println!("unknown operator");
            continue;
        }

        let ans = match operator {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => num1 / num2,
            _ => panic!("error in operator"), // interesting didn't know about this
        };
        println!("{} {} {} = {}\n", num1, operator, num2, ans);
    }
}

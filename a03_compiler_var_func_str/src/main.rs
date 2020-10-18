fn main() {
    let _a: i32 = 5;

    let _b: [i32; 3] = [1, 2, 3];

    let _c: [&str; 2] = ["dustin", "lo"];

    let num1: i32 = 3;
    println!("num1: {}", num1);
    let num1: f32 = 3.3;
    println!("num1: {}", num1);
    let num1 = 3; // compiler can automatically assign variable type
    println!("num1: {}", num1);

    // int (i32) array
    let _v: Vec<i32> = Vec::new();

    // mutable variables
    let mut num2 = 45; // mut tells compiler you're going to change value later
    println!("num2: {}", num2);
    num2 = 55;
    println!("num2: {}", num2);

    // immutable literal stored in read-only data
    let name1: &'static str = "engineer man";
    println!("name1: {}", name1);

    let mut name2: String = String::new();
    name2.push_str("engineer man");
    // can also do it like this:   name2 = name2 + name1;
    println!("name2: {}", name2);

    // string slices (retrieving parts of a string)
    let name3: &str = &name1[..8];
    println!("name3: {}", name3);
    let name4: &str = &name1[9..];
    println!("name4: {}", name4);

    println!("\nlooping over string:");
    for c in name2.chars() {
        println!("{}", c);
    }

    let name = String::from("engineer man");
    let name_len = get_length(&name); // type usize
    println!("{}, len: {}", name, name_len);

    borrow_name(&name);
    println!("{}", name);

    let example_sum = add_numbers(32, 74);
    println!("example_sum: {}", example_sum);

    let mut num = 25;
    increment(&mut num);
}

// NOTE: LOOKS LIKE WE PASS POINTERS ONLY TO FUNCTIONS, NOT THE ACTUAL VALUE
// IT DE-ALLOCATES THE MEMORY AFTER THE FUNCTION FINISHES USING IT

fn get_length(s: &str) -> usize {
    return s.chars().count();
}

fn borrow_name(s: &String) {
    println!("taking ownership");
    println!("{}", s);
}

fn add_numbers(i1: i32, i2: i32) -> i32 {
    return i1 + i2;
}

fn increment(val: &mut i32) {
    *val = *val + 1;
}

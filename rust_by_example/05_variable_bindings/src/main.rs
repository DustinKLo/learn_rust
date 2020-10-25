fn main() {
    // SCOPING AND SHADOWING
    let long_lived_binding = 1;
    {
        let short_lived_binding = 2; // cant use short_lived_binding outside block
        println!("inner short: {}", short_lived_binding);

        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }
    println!("outer long: {}", long_lived_binding);

    let long_lived_binding = 'a';
    println!("outer long: {}", long_lived_binding);
    println!("");

    // DECLARE FIRST
    let a_binding;
    {
        let x = 2;
        a_binding = x * x;
    }
    println!("a binding: {}", a_binding);

    let another_binding;
    another_binding = 1;
    println!("another binding: {}", another_binding);

    // FREEZING
    let mut _mutable_integer = 7i32;
    {
        let _mutable_integer = _mutable_integer;
        // _mutable_integer = 50; // because inner scope is not mutable, will break when re-assigning
    }
    _mutable_integer = 3;
    println!("_mutable_integer: {}", _mutable_integer);
}

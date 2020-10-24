fn main() {
    enum Color {
        Red,   // 0
        Green, // 1
        Blue,
        Orange,
        Custom(String),           // tuple struct style
        Corrd { x: i32, y: i32 }, // classic struct style
    }
    // println!("Red: {}", Color::Red as i32);
    // println!("Green: {}", Color::Green as i32);
    // println!("Blue: {}", Color::Blue as i32);
    // println!("Orange: {}", Color::Orange as i32);

    enum Number {
        One = 1,
        Five = 5,
        Ten = 0xA,
    }
    println!("");
    println!("One: {}", Number::One as i32);
    println!("Five: {}", Number::Five as i32);
    println!("Ten: {}", Number::Ten as i32);
    println!("");

    let favorite_color: Color = Color::Green;
    let custom: Color = Color::Custom("pink".to_string());

    // check with if let
    if let Color::Green = favorite_color {
        println!("favorite color is green");
    }

    // check with match
    match favorite_color {
        Color::Green => println!("favorite color is green"),
        Color::Blue => println!("favorite color is blue"),
        _ => {} // match requires all variants, "_ =>" handles the rest
    }

    match custom {
        Color::Custom(color) => println!("custom color: {}", color),
        _ => {}
    }

    println!("");

    // built-in Option<T> enum
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let mut age: Option<i32> = None;
    age = Some(19);
    // can also do "let age: Option<i32> = Some(19);"

    match age {
        Some(age) => {
            if age >= 21 {
                println!("person is old enough to drink");
            } else {
                println!("person is NOT old enough to drink");
            }
        }
        None => println!("unknown age"),
    }
}

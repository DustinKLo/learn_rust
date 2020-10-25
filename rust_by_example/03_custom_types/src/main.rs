#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Unit; // unit struct
struct Pair(i32, f32); // tuple struct

// struct with fields (similar to go structs or python dictionaries)
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn area(&self) -> f32 {
        // using struct deconstructing get struct values
        let Point { x: x1, y: y1 } = &self.top_left;
        let Point { x: x2, y: y2 } = &self.bottom_right;
        return ((x2 - x1) * (y2 - y1)).abs() as f32;
    }
}

fn main() {
    let name = "Dustin";
    let age = 28;
    let dustin = Person { name, age };
    println!("{:?}", dustin);

    let point = Point { x: 69.3, y: 42.0 };
    println!("{:?}", point);

    // let point_2 = Point { x: 54.0, ..point };
    let point_2 = Point { x: 54.1, y: 100.6 };
    println!("{:?}", point_2);

    let _rectangle = Rectangle {
        top_left: point,
        bottom_right: point_2,
    };
    println!("area: {}", _rectangle.area());

    let _unit = Unit;
    let _pair = Pair(69, 420.0);
    let Pair(_integer, _decimal) = _pair; // deconstructing a struct (similar to javascript)
}

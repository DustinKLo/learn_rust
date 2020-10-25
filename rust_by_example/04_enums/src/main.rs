#![allow(dead_code)]
use crate::List::Nil;
use crate::List::Cons;

enum WebEvent {
    PageLoad, // an enum may either be 'unit-like'
    PageUnload,
    KeyPress(char), // like tuple structs
    Paste(String),
    Click { x: i64, y: i64 }, // or c-like structures
}

// function takes a WebEvent enum as arg and returns nothing
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page un-loaded"),
        WebEvent::KeyPress(c) => println!("pressed: {}", c),
        WebEvent::Paste(s) => println!("pasted: {}", s),
        WebEvent::Click { x, y } => println!("clicked at {}, {}", x, y),
    }
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

// linked list (using enum)
enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        return Nil; // Nil has type List
    }
    // consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        return Cons(elem, Box::new(self)); // Cons also has type List
    }

    // recusively getting the length of the linked list
    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    // recursively printing the linked list
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => format!("{} -> {}", head, tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned()); // `to_owned()` creates an owned `String` from a string slice.
    let click = WebEvent::Click { x: 69, y: 420 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;
    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
    println!("");

    use crate::Status::{Poor, Rich}; // use each name so they are available without manual scoping
    use crate::Work::*; // automatically use each name inside 'Work'

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilians work"),
        Soldier => println!("Soldiers fight"),
    }

    println!("");
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    list = list.prepend(4);
    list = list.prepend(5);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

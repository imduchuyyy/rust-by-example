// // An attribute to hide warnings for unused code.
// #![allow(dead_code)]

// use std::{fmt::Display};

// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u8,
// }

// // A unit struct
// struct Unit;

// // A tuple struct
// struct Pair(i32, f32);

// // A struct with two fields
// struct Point {
//     x: f32,
//     y: f32,
// }

// // Structs can be reused as fields of another struct
// struct Rectangle {
//     // A rectangle can be specified by where the top left and bottom right
//     // corners are in space.
//     top_left: Point,
//     bottom_right: Point,
// }

// impl Display for Rectangle {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "Rectangle [top_left: ({}, {}), bottom_right: ({}, {})]",
//             self.top_left.x, self.top_left.y, self.bottom_right.x, self.bottom_right.y
//         )
//     }
// }

// fn rect_area(rect: &Rectangle) -> f32 {
//     let width = rect.bottom_right.x - rect.top_left.x;
//     let height = rect.bottom_right.y - rect.top_left.y;
//     width * height
// }

// fn square(point: Point, side: f32) -> Rectangle {
//     Rectangle {
//         top_left: Point {
//             x: point.x,
//             y: point.y,
//         },
//         bottom_right: Point {
//             x: point.x + side,
//             y: point.y + side,
//         },
//     }
// }

// fn main() {
//     // Create struct with field init shorthand
//     let name = String::from("Peter");
//     let age = 27;
//     let peter = Person { name, age };

//     // Print debug struct
//     println!("{:?}", peter);

//     // Instantiate a `Point`
//     let point: Point = Point { x: 5.2, y: 0.4 };
//     let another_point: Point = Point { x: 10.3, y: 0.2 };

//     // Access the fields of the point
//     println!("point coordinates: ({}, {})", point.x, point.y);

//     // Make a new point by using struct update syntax to use the fields of our
//     // other one
//     let bottom_right = Point { x: 10.3, ..another_point };

//     // `bottom_right.y` will be the same as `another_point.y` because we used that field
//     // from `another_point`
//     println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

//     // Destructure the point using a `let` binding
//     let Point { x: left_edge, y: top_edge } = point;

//     let rectangle = Rectangle {
//         // struct instantiation is an expression too
//         top_left: Point { x: left_edge, y: top_edge },
//         bottom_right: bottom_right,
//     };

//     // Instantiate a unit struct
//     let _unit = Unit;

//     // Instantiate a tuple struct
//     let pair = Pair(1, 0.1);

//     // Access the fields of a tuple struct
//     println!("pair contains {:?} and {:?}", pair.0, pair.1);

//     // Destructure a tuple struct
//     let Pair(integer, decimal) = pair;

//     println!("pair contains {:?} and {:?}", integer, decimal);

//     let area = rect_area(&rectangle);
//     println!("The area of the rectangle is {}", area);

//     let square_rect = square(point, 3.0);
//     println!("Square: {}", square_rect);
// }


// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

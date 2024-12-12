#![allow(dead_code)]

#[derive(Debug)]
struct Persion {
    name: String,
    age: u8,
}
struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let name = String::from("Peter");
    let age = 27u8;
    let peter = Persion{name, age};
    println!("peter: {:?}", peter);
    let point: Point = Point {x: 5.2, y: 0.4};
    let another_ppint: Point = Point{x: 10.1, y: 0.2};
    println!("point {}, {}", point.x, point.y);

    let bottom_right = Point { x: 10.3, ..another_ppint};
    println!("second point: {}, {}", bottom_right.x, bottom_right.y);
    let Point { x: left_edge, y: top_edge} = point;

    let _rectangle = Rectangle {
        top_left: Point {x: left_edge, y: top_edge},
        bottom_right: bottom_right,
    };
    let _unit = Unit;

    let pair = Pair(1, 1.1);
    println!("pair {:?}, {:?}", pair.0, pair.1);
    let Pair(integer, decimal) = pair;
    println!("{:?}, {:?}", integer, decimal);
}
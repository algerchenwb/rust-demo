#![allow(dead_code)]

enum Number {
    Zero,
    One,
    Two,

}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}
fn main() {
    use crate::Number::*;
    use crate::Color::*;
    println!("zero is {}", Zero as i32);
    println!("one is {}", One as u8);
    println!("red is #{:08x}", Red as i32);
    println!("violets are #{:06x}", Blue as i32);

}
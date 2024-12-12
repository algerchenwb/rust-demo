#![allow(unreachable_code, unused_labels)]

fn main() {
    'outer: loop {
        println!("outer");
        'inner: loop {
            println!("inner");
            break 'outer;
        }
        println!("unreachable");
    }
    println!("reached");



    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result is {}", result)
}
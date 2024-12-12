fn main() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1;
    let f = 1.0;

    println!("size of x is {}", std::mem::size_of_val(&x));
    println!("size of y is {}", std::mem::size_of_val(&y));
    println!("sizeo {}, {}, {}", std::mem::size_of_val(&z), std::mem::size_of_val(&i), std::mem::size_of_val(&f));
}
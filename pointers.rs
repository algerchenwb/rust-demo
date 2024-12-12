fn main() {
    let reference = &4;
    match reference {
        &u32 => println!("Got a i32 value vis matching: "),
        &val => println!("Got a value vis dereferencing: {:?}", val),
    
    }
    match *reference {
        val => println!("Got a value vis matching: {:?}", val),
    }

}
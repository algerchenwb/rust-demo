fn main() {
    let number = 13;

    println!("Tell me about {}", number);
    match number {
        1..=13 => println!("This is one to thirteenth"),
        _ => println!("This is something else"),
    }
    let boolean = true;;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);
}
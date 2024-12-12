fn main() {
    let n = 5;
    let big_n =
        if n < 10 && n > -10 {
            println!("A small number");
             10 * n
        } else {
            println!("A big number");
            n / 2
        };
        println!("{} -> {}", n, big_n);
}
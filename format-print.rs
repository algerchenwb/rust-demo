fn main() {
    println!("{today} today", today="2024-12-09");
    println!("{0}, this is {1}. {1}, this is {0}", "jeff", "kgong");
    println!("{subject} {verb} {object}",
            subject= "the lazy dog",
            verb="the quick brown fox",
            object="jups over");
    println!("Base 10:   {}", 7898787);
    println!("Base 2 (binary): {:b}", 7898787);
    println!("Base 8 (octal): {:o}", 7898787);
    println!("Base 16 (hexadecimal): {:x}", 7898787);

    println!("{number:>10}", number= 1);
    println!("{number:0>10}", number=1);
    println!("{number:0<5}", number=1);
    println!("{number:0>width$}", number=1, width=5);

    #[allow(dead_code)]
    struct Structure(i32);

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
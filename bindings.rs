

fn main() {
    let an_integer = 1u32;

    // an_integer += 1;
    let a_boolean = true;
    let unit = ();

    let mut copied_integer = an_integer;
    copied_integer += 1;

    println!("An integer {}, {}", an_integer, copied_integer);
    let an_integer = ();

}
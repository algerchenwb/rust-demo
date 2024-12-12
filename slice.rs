use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice:{}", slice[0]);
    println!("len{}", slice.len());
}
fn main() {
    let xs: [i32;5] = [1,2,3,4,5];
    let ys: [i32; 500] = [0;500];

    println!("first element of arrya: {}", xs[0]);
    println!("second element of array:{}", xs[xs.len() - 1]);

    println!("array occupies {} bytes", mem::size_of_val(&xs));
    analyze_slice(&xs);

    analyze_slice(&ys[1 .. 4]);

    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}:{}", i, xval),
            None => println!("Slow down {} is too far", i),
        }
    }
}
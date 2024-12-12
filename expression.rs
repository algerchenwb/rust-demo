fn main() {
    let x = 5i32;
    let y = {
        let x_qu = x * x;
        let x_cub = x_qu * x;
        x_cub + 1
    
    };
    let z = 1i32;
    let z = {
        2 * x;
    };
    println!("x: {}, y: {}, z: {}", x, y, z);
}
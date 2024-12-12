#[allow(dead_code)]

enum Color {
    RGB(i32, i32, i32),
    HSV(i32, i32, i32),
}
fn main() {
    let teripe = (1, true);
    match teripe {
        (1, boolean) => println!("boolean: {}", boolean),
        (x, y) => {
            println!("x: {}, y: {}", x, y);
     
        }
    
    }
    let color = Color::RGB(1, 2, 3);
    match color {
        Color::RGB(r, g, b) => {
            println!("r: {}, g: {}, b: {}", r, g, b);
        }
        Color::HSV(h, s, v) => {
            println!("h: {}, s: {}, v: {}", h, s, v);
        },
    }


}
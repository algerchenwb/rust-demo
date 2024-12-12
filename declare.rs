fn main() {
    let a_binding;

   let mut _mutable_interge = 7i32;
   {
    let _mutable_interge;
    _mutable_interge = 50;
    println!("inside: {}", _mutable_interge);
   }
   _mutable_interge = 3;
   println!("outside: {}", _mutable_interge);
    a_binding = 1;
    println!("after: {}", a_binding);
}
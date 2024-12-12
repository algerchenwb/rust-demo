#[allow(dead_code)]

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

fn main() {
    use crate::Stage::{Beginner, Advanced};
    use crate::Role::*;

    let stage = Stage::Beginner;
    let role = Student;

    match stage {
        Beginner => println!("Beginner"),
        Advanced => println!("Advanced"),
    }   
    match role {
        Student => println!("Student"),
        Teacher => println!("Teacher"),
    }
    
     }

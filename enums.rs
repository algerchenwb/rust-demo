enum WebEvent {
    PageLoad,
    PageUnLoad,
    KeyPress(char),
    Parste(String),
    Click{x: i64, y: i64},
}

enum VeryVerbose {
    Add,
    Subtract,
}

type Operations = VeryVerbose;

impl VeryVerbose {
    fn str(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnLoad => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed {}", c),
        WebEvent::Parste(s) => println!("parsed string: {}", s),
        WebEvent::Click {x, y} => {
            println!("clicked at x = {}, y ={}", x, y);
        }
    }
}

fn main() {
    let press = WebEvent::KeyPress('x');
    let pasted = WebEvent::Parste("my text".to_owned());
    let click = WebEvent::Click {x: 20, y: 80};
    inspect(press);
    inspect(pasted);
    inspect(click);
    let x = Operations::Add;
    let sum: i32 = x.str(1,2);
    println!("sum: {}", sum);
}
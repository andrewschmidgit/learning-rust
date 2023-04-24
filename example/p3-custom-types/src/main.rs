fn main() {
    structs::main();
    enums::main();
    constants::main();
}

fn line() {
    println!("-----------------------------");
}

fn title(name: &str) {
    println!("");
    line();
    println!("> {}", name.to_uppercase());
    println!("");
}

mod structs {
    #![allow(dead_code)]
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    struct Unit;
    struct Pair(i32, f32);

    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }

    #[derive(Debug)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    impl Rectangle {
        fn area(&self) -> f32 {
            let Rectangle {
                top_left: tl,
                bottom_right: br,
            } = self;

            let x = (tl.x - br.x).abs();
            let y = (tl.y - br.y).abs();

            x * y
        }
    }

    fn square(point: Point, side_length: f32) -> Rectangle {
        let br = Point {
            x: point.x + side_length,
            y: point.y + side_length,
        };

        Rectangle {
            top_left: point,
            bottom_right: br,
        }
    }

    pub fn main() {
        crate::title("structs");
        let name = String::from("Peter");
        let age = 27;
        let peter = Person { name, age };

        println!("{:?}", peter);

        let point: Point = Point { x: 10.3, y: 0.4 };

        println!("point coordinates: ({}, {})", point.x, point.y);

        let bottom_right = Point { x: 5.2, ..point };

        println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

        let Point {
            x: left_edge,
            y: top_edge,
        } = point;

        let _rectangle = Rectangle {
            top_left: Point {
                x: left_edge,
                y: top_edge,
            },
            bottom_right: Point {
                x: bottom_right.x,
                y: 10.0,
            },
        };

        println!("Rectangle {:?} has area: {}", _rectangle, _rectangle.area());

        let square = square(point, 5.2);
        println!("square {:?} has area: {}", square, square.area());

        let _unit = Unit;

        let pair = Pair(1, 0.1);

        println!("pair contains {:?} and {:?}", pair.0, pair.1);

        let Pair(integer, decimal) = pair;

        println!("pair contains {:?} and {:?}", integer, decimal);
        crate::line();
    }
}

mod enums {
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }

    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\"", s),
            WebEvent::Click { x, y } => println!("clicked at x={} y={}.", x, y),
        }
    }

    enum VeryVerboseNameSuperLongDontWantToType {
        Add,
        Subtract,
    }

    impl VeryVerboseNameSuperLongDontWantToType {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y, // Self is a built in alias
                Self::Subtract => x - y,
            }
        }
    }

    type Operations = VeryVerboseNameSuperLongDontWantToType;

    use List::*;
    enum List {
        Cons(u32, Box<List>),
        Nil,
    }

    impl List {
        fn new() -> List {
            Nil
        }

        fn prepend(self, elem: u32) -> List {
            Cons(elem, Box::new(self))
        }

        fn len(&self) -> u32 {
            match *self {
                Cons(_, ref tail) => 1 + tail.len(),
                Nil => 0,
            }
        }

        fn stringify(&self) -> String {
            match *self {
                Cons(head, ref tail) => {
                    format!("{}, {}", head, tail.stringify())
                }
                Nil => format!("nil"),
            }
        }
    }

    fn list() {
        let mut list = List::new();

        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);

        println!("linked list has length: {}", list.len());
        println!("{}", list.stringify());
    }

    pub fn main() {
        crate::title("enums");
        let pressed = WebEvent::KeyPress('x');
        let pasted = WebEvent::Paste("My text".to_owned());
        let click = WebEvent::Click { x: 20, y: 70 };
        let load = WebEvent::PageLoad;
        let unload = WebEvent::PageUnload;

        inspect(pressed);
        inspect(pasted);
        inspect(click);
        inspect(load);
        inspect(unload);

        let x = Operations::Add;

        // Can also use "use" to bring in enum values
        use VeryVerboseNameSuperLongDontWantToType::{Add, Subtract};
        use WebEvent::*;

        let new_event = KeyPress('x');
        let operation = Add;

        list();
        crate::line();
    }
}

mod constants {
    const THRESHOLD: i32 = 10;
    static LANGUAGE: &str = "Rust";
    fn is_big(n: i32) -> bool {
        n > THRESHOLD
    }

    pub fn main() {
        crate::title("constants");
        let n = 16;
        println!("This is {}", LANGUAGE);
        println!("The threshold is {}", THRESHOLD);
        println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
        crate::line();
    }
}

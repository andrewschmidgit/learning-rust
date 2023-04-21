use std::fmt::Display;

fn main() {
    println!("Hello, world!");

    let x = 5 + /* 90 + */ 5;
    println!("{x}");

    println!("{} days", 31);
    println!("{0} this is {1}, {1}, this is {0}", "Alice", "Bob");

    println!(
        "{subject}, {verb}, {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!("Base 10: {}", 69420);
    println!("Base 2: {:b}", 69420);
    println!("Base 10: {:o}", 69420);
    println!("Base 10: {:x}", 69420);
    println!("Base 10: {:X}", 69420);

    println!("{number:>5}", number = 1);
    println!("{number:0<5}", number = 1);
    println!("{number:0>width$}", number = 1, width = 5);

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    println!("This struct `{}` prints now!", Structure(3));

    let pi = 3.14159265;
    println!("Pi is rougly {:>3.3}", pi);

    println!("{:#?}", Deep(Structure(3)));

    let min_max = MinMax(30, 90);
    println!("Debug: {:?}", min_max);
    println!("Debug: {}", min_max);

    let complex_number = Complex {
        real: 3.3,
        imag: 7.2,
    };

    println!("{}", complex_number);
    println!("{:?}", complex_number);

    let v = List(vec![1, 2, 3]);

    println!("{}", v);
}

#[derive(Debug)]
struct Structure(i32);

impl Display for Structure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

//struct UnPrintable(i32);

#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct MinMax(i64, i64);

impl Display for MinMax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.0, self.1)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

struct List(Vec<i32>);
impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (index, v) in vec.iter().enumerate() {
            if index != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", index, v)?;
        }

        return write!(f, "]");
    }
}

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

    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", *city);
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        println!("{}", *color);
    }
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

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lat >= 0.0 { 'E' } else { 'W' };

        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RGB ({}, {}, {}) 0x{:0>2x}{:0>2x}{:0>2x}",
            self.red, self.green, self.blue, self.red, self.green, self.blue,
        )
    }
}

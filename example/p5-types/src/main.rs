#![allow(overflowing_literals)]
fn main() {
    casting();
    literals();
    inference();
    aliasing();
}

fn casting() {
    let decimal = 65.4321_f32;

    // no implicit conversion
    // let integer: u8 = decimal;

    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u16 is: {}", 1000 as u16);
    println!("1000 as a u8 is: {}", 1000 as u8);
    println!("  -1 as a u8 is: {}", (-1i8) as u8);

    println!("1000 % 256 as a u8 is: {}", 1000 % 256);

    println!(" 128 as a i16 is: {}", 128 as i16);
    println!(" 128 as a i8 is : {}", 128 as i8);

    println!("1000 as a u8 is : {}", 1000 as u8);
    println!(" 232 as a u8 is : {}", 232 as i8);

    println!(" 300.0 as a u8 is : {}", 300.0_f32 as u8);
    println!("-100.0 as a u8 is : {}", -100.0_f32 as u8);
    println!("   nan as a u8 is : {}", f32::NAN as u8);
}

fn literals() {
    let x = 1u8;
    let y = 2u32;
    let z = 3u32;

    let i = 1;
    let f = 1.0;

    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

fn inference() {
    let elem = 5u8;

    let mut vec = Vec::new();

    // vec now knows it's a vec of u8s
    vec.push(elem);

    println!("{:?}", vec);
}

type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn aliasing() {
    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}

fn numbers() {
    let x = 2.0; // f64

    let y: f32 = 3.0;

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // -1

    let remainder = 43 % 5;
}

fn boolean() {
    let t = true;
    let f: bool = false;
}

fn characters() {
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
}

fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    println!("The value of y is also: {}", tup.1);
}

fn array() {
    // [type; # of elements]
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // [initial value per element; # of elements]
    let b = [3; 5];

    let first = b[0];
    let second = b[1];

    // let error = b[3];
}

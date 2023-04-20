pub fn run() {
    create();

    let mut s = String::from("hello");
    update(&mut s);
    println!("{s}");

    interpolation();
    iterate();
}

fn create() {
    let s = "initial".to_string();
    let s = String::from("initial");
    let s = String::from("😄");
}

fn update(s: &mut String) {
    s.push_str(" world");

    let s1 = String::from("Hello ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    // s1 had ownership taken away

    println!("{s3}");
}

fn interpolation() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("{s}");
}

fn iterate() {
    let s = String::from("h†¨ˆøπ¬˚∆˙©ƒ∂ß");
    for c in s.chars() {
        print!("{c}");
    }

    println!("");
}

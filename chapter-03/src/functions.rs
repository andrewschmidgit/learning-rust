fn main() {
    println!("Hello world");

    another_function(4);
    five();
}

fn another_function(x: i32) {
    println!("Another function with value {x}");

    // expression
    let scope_block = {
        let x = 3;
        x + 1 // no semicolon, returns
    };
}

fn five() -> i32 {
    5
}

// invalid because of the semicolon
//fn five() -> i32 {
//    5;
//}

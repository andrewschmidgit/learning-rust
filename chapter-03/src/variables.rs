fn main() {
    compiler_error();
    mutability();
}

fn immutable() {
    let x = 5;
    println!("The value of x is {x}");
    x = 6; // produces compiler error
    println!("The value of x is {x}");
}

fn mutability() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
}

fn constants() {
    const ONE_DAY_IN_SECONDS: u32 = 24 * 60 * 60;
}

fn shadowing() {
    let x = 5; // immutable
    let x = x + 1; // still immutable

    {
        let x = x * 2;
        // Prints: 12
        println!("The value of x in the inner scope is: {x}");
    }

    // Prints: 6
    println!("The value of x is: {x}");

    // Shadowing can change the type to avoid renaming
    let spaces = "   ";
    let spaces = spaces.len();

    let mut spaces2 = "   ";
    // Can't change the type
    spaces2 = spaces2.len();
}

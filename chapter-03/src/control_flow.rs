fn main() {
}

fn ifs() {
    let number = 5;

    if number < 5 {
        println!("wow true");
    } else {
        println!("wow true");
    }

    let inline = if number == 5 { "was 5 big wow" } else { "big sad no 5" };
}

fn unconditional_loops() {
    let mut counter = 0;
    loop {
        println!("wow such loop");
        counter += 1;

        if counter = 5 { break }
    }

    let watch_this = loop { 
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("watch_this = {watch_this}"); // should be 20

    let x_max = 10;
    let y_max = 15;
    let mut x_position = 0;
    'outer: loop {
        let mut y_position = 0;

        'inner: loop {
            if y_position == y_max {
                println!("overflowing y");
                break;
            }

            if x_position == x_max {
                println!("overflowing x");
                break 'outer;
            }

            y_position += 1;
        }

        x_position += 1;
    }
}

fn conditional_loops() {
    let mut number = 3;

    while number != 0 {
        println!("number: {number}");
        number -= 1;
    }

    println!("woah dude. while oops");

    let genders = [true, false];
    let index = 0;

    while index < 2 {
        let gender = genders[index];
        println!("gender {index}: {gender}");
        index += 1;
    }

    for gender in genders {
        println!("gender: {gender}");
    }
    
    // cleaner countdown
    for number in (1..4).rev() {
        println!("{number}!");
    }
}

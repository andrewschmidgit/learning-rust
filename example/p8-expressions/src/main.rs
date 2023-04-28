fn main() {
    ifelse::main();
    loops::main();
    whiling::main();
    for_range::main();
    matching::main();
    if_let::main();
    let_else::main();
    while_let::main();
}

fn line(title: Option<&str>) {
    println!("");
    println!("-------------------------------");

    if let Some(t) = title {
        println!("{}", t);
    }

    println!("");
}

mod ifelse {
    pub fn main() {
        crate::line(Some("If-Else"));
        let n = 5;

        if n < 0 {
            print!("{} is negative", n);
        } else if n > 0 {
            print!("{} is positive", n);
        } else {
            print!("{} is 0", n);
        }

        let big_n =
            if n < 10 && n > -10 {
                println!(", and is a small number, increase ten-fold");
                10 * n
            } else {
                println!(", and is a big number, halve the number");

                n / 2
            };

        println!("{} -> {}", n, big_n);
    }
}

mod loops {
    pub fn main() {
        crate::line(Some("Loops"));
        let mut count = 0u32;

        println!("Let's count until infinity!");

        loop {
            count += 1;

            if count == 3 {
                println!("three");

                continue;
            }

            println!("{}", count);

            if count == 5 {
                println!("Ok, that's enough");

                break;
            }
        }

        crate::line(None);

        'outer: loop {
            println!("Entered the outer loop");

            'inner: loop {
                println!("Entered the inner loop");

                break 'outer;
            }

            println!("This point will never be reached");
        }

        println!("Exited the outer loop");

        crate::line(None);
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("Counter: {}, {}", counter, result == 20);
    }
}

mod whiling {
    pub fn main() {
        crate::line(Some("Whiling"));
        let mut n = 1;

        while n < 101 {
            if n % 15 == 0 {
                println!("fizzbuzz");
            } else if n % 3 == 0 {
                println!("fizz");
            } else if n % 5 == 0 {
                println!("buzz");
            } else {
                println!("{}", n);
            }

            n += 1;
        }
    }
}

mod for_range {
    pub fn main() {
        crate::line(Some("For and Range"));
        for n in 1..101 {
            if n % 15 == 0 {
                println!("fizzbuzz");
            } else if n % 3 == 0 {
                println!("fizz");
            } else if n % 5 == 0 {
                println!("buzz");
            } else {
                println!("{}", n);
            }
        }

        crate::line(None);

        for n in 1..=101 {
            if n % 15 == 0 {
                println!("fizzbuzz");
            } else if n % 3 == 0 {
                println!("fizz");
            } else if n % 5 == 0 {
                println!("buzz");
            } else {
                println!("{}", n);
            }
        }

        crate::line(None);

        let names = vec!["Bob", "Frank", "Ferris"];

        for name in names.iter() {
            match name {
                &"Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }

        println!("names: {:?}", names);
        crate::line(None);

        let names = vec!["Bob", "Frank", "Ferris"];

        for name in names.into_iter() {
            match name {
                "Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }

        // Can't do this now, was consumed
        // println!("names: {:?}", names);
        crate::line(None);

        let mut names = vec!["Bob", "Frank", "Ferris"];

        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean among us!",
                _ => "Hello",
            }
        }

        println!("names: {:?}", names);
    }
}

mod matching {
    fn destructuring() {
        crate::line(Some("Destructuring"));

        let triple = (0, -2, 3);
        println!("Tell me about {:?}", triple);

        match triple {
            (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
            (1, ..) => println!("First is `1` and the rest doesn't matter"),
            (.., 2) => println!("Last is `2` and the rest doesn't matter"),
            (3, .., 4) => println!("First is `3`, last is `4`, and the rest doesn't matter"),
            _ => println!("It doesn't matter what they are"),
        }


    }

    pub fn main() {
        crate::line(Some("Matching"));

        let number = 13;
        println!("Tell me about {}", number);

        match number {
            1 => println!("One!"),
            2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
            13..=19 => println!("A teen"),
            _ => println!("Ain't special")
        }

        let boolean = true;
        let binary = match boolean {
            true => 1,
            false => 0,
        };

        println!("{} -> {}", boolean, binary);
    }
}

mod if_let {
    pub fn main() {

    }
}

mod let_else {
    pub fn main() {

    }
}

mod while_let {
    pub fn main() {

    }
}

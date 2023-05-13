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
    #[allow(dead_code)]
    enum Color {
        Red,
        Blue,
        Green,
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    fn destructuring() {
        crate::line(Some("Destructuring"));

        // Tuples
        let triple = (0, -2, 3);
        println!("Tell me about {:?}", triple);

        match triple {
            (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
            (1, ..) => println!("First is `1` and the rest doesn't matter"),
            (.., 2) => println!("Last is `2` and the rest doesn't matter"),
            (3, .., 4) => println!("First is `3`, last is `4`, and the rest doesn't matter"),
            _ => println!("It doesn't matter what they are"),
        }

        // Arrays
        let array = [1, -2, 6];

        match array {
            [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
            [1, _, third] => println!("array[0] = 1, array[2] = {} and array[1] was ignored", third),
            [-1, second, ..] => println!("array[0] = -1, array[1] = {} and all the other ones were ignored", second),
            [3, second, tail @ ..] => println!("array[0] = 3, array[1] = {} and the other elements were {:?}", second, tail),
            [first, middle @ .., last] => println!("array[0] = {}, middle = {:?}, array[2] = {}", first, middle, last),
        };

        // Enums
        let color = Color::RGB(122, 17, 40);

        println!("What color is it?");

        match color {
            Color::Red => println!("The color is Red!"),
            Color::Blue => println!("The color is Blue!"),
            Color::Green => println!("The color is Green!"),
            Color::RGB(r, g, b) => println!("Red: {}, green: {}, blue: {}!", r, g, b),
            Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
            Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
            Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
            Color::CMYK(c, m, y, k) => println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!", c, m, y, k),
        }

        // Pointers / ref
        let reference = &4;
        match reference {
            &val => println!("Got a value via destructuring: {:?}", val),
        }
        
        match *reference {
            val => println!("Got a value via destructuring: {:?}", val),
        }

        let not_a_reference = 3;
        let ref is_a_reference = 3;

        let value = 5;
        let mut mut_value = 6;
        
        match value {
            ref r => println!("Got a reference to a value: {:?}", r),
        }

        match mut_value {
            ref mut m => {
                // Got a reference. Gotta dereference it before we can
                // add anything to it.
                *m += 10;
                println!("We added 10. `mut_value`: {:?}", m);
            }
        }

        // Structs
        struct Foo {
            x: (u32, u32),
            y: u32,
        }

        let foo = Foo { x: (1, 2), y: 3 };
        match foo {
            Foo { x: (1, b), y } => println!("Fist of x is 1, b = {}, y = {}", b, y),
            Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
            Foo { y, .. } => println!("y = {}, we don't care about x", y),
        }
    }
    
    enum Temperature {
        Celsius(i32),
        Fahrenheit(i32)
    }

    fn guards() {
        crate::line(Some("Guards"));

        let temperature = Temperature::Celsius(35);

        match temperature {
            Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
            Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),
            Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
            Temperature::Fahrenheit(t) => println!("{}F is below 86 Fahrenheit", t),
        }

        let number: u8 = 4;

        match number {
            i if i == 0 => println!("Zero"),
            i if i > 0 => println!("Greater than zero"),
            _ => println!("Should never happen"),
        }
    }

    fn age() -> u32 { 15 }
    fn some_number() -> Option<u32> { Some(42) }
    fn binding() {
        println!("Tell me what type of person you are");

        match age() {
            0 => println!("I haven't celebrated my first birthday yet"),
            n @ 1 ..= 12 => println!("I'm a child of age {:?}", n),
            n @ 13 ..= 19 => println!("I'm a tenn of age {:?}", n),
            n => println!("I'm an old of age {:?}", n),
        }

        match some_number() {
            Some(n @ 42) => println!("The Answer: {}!", n),
            Some(n) => println!("Not interesting... {}!", n),
            _ => {},
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

        destructuring();
        guards();
        binding();
    }
}

mod if_let {
    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }

    pub fn main() {
        // Funky
        let optional = Some(7);

        match optional {
            Some(i) => println!("This is a really long string and `{:?}`", i),
            _ => {}
        }

        // Better
        let number = Some(7);
        let letter: Option<i32> = None;
        let emoticon: Option<i32> = None;

        if let Some(i) = number { println!("Matched {:?}!", i) }
        if let Some(i) = letter { 
            println!("Matched {:?}!", i);
        } else {
            println!("Didn't match a number. Let's go with a letter!");
        }

        let i_like_letters = false;
        if let Some(i) = emoticon { 
            println!("Matched {:?}!", i);
        } else if i_like_letters {
            println!("Didn't match a number. Let's go with a letter!");
        } else {
            println!("I don't like letters. Let's go with an emoticon :)!");
        }

        // Enums
        let a = Foo::Bar;
        let b = Foo::Baz;
        let c = Foo::Qux(100);

        if let Foo::Bar = a { println!("a is foobar"); }
        if let Foo::Bar = b { println!("b is foobar"); }
        if let Foo::Qux(value) = c { println!("c is {}", value); }
        if let Foo::Qux(value @ 100) = c { println!("c is 100"); }

        if let Foo::Bar = a {
            println!("a is foobar"); 
        }
    }
}

mod let_else {
    use std::str::FromStr;
    fn get_count_item(s: &str) -> (u64, &str) {
        let mut it = s.split(' ');
        let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
            panic!("Can't segment count item pair: '{s}'");
        };

        let Ok(count) = u64::from_str(count_str) else {
            panic!("Can't parse integer: '{count_str}'");
        };

        (count, item)
    }
    pub fn main() {
        assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
    }
}

mod while_let {
    pub fn main() {
        let mut optional = Some(0);

        // No
        loop {
            match optional {
                Some(i) => {
                    if i > 9 {
                        println!("Greater than 9, quit!");
                        optional = None;
                    } else {
                        println!("`i` is `{:?}`. Try again.", i);
                        optional = Some(i + 1);
                    }
                },
                _ => { break; }
            }
        }

        // Yes
        while let Some(i) = optional {
            if i > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("`i` is `{:?}`. Try again.", i);
                optional = Some(i + 1);
            }
        }
    }
}

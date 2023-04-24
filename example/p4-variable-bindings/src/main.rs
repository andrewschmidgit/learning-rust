fn line() {
    println!("--------------------------");
}

fn title(name: &str) {
    println!("");
    line();
    println!("> {}", name.to_uppercase());
    println!("");
}

fn main() {
    intro();
    mutability::main();
    scope_and_shadowing::main();
    declare_first::main();
    freezing::main();
}

fn intro() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    let _unused_variable = 3u32;
    let noisy_unused_variable = 2u32;
}

mod mutability {
    pub fn main() {
        crate::title("mutability");
        let _immutable_binding = 1;
        let mut mutable_binding = 1;

        println!("Before mutation: {}", mutable_binding);
        mutable_binding += 1;
        println!("After mutation: {}", mutable_binding);

        // Can't do _immutable_binding += 1;
        crate::line();
    }
}

mod scope_and_shadowing {
    pub fn main() {
        crate::title("scope and shadowing");
        let long_lived_binding = 1;

        {
            let short_lived_binding = 2;

            println!("inner short: {}", short_lived_binding);
        }

        // Can't do this
        //println!("outer short: {}", short_lived_binding);

        println!("outer long: {}", long_lived_binding);

        let shadowed_binding = 1;
        {
            println!("before being shadowed: {}", shadowed_binding);
            let shadowed_binding = "abc";

            println!("shadowed in inner block: {}", shadowed_binding);
        }

        println!("outside inner block: {}", shadowed_binding);

        let shadowed_binding = 2;
        println!("shadowed in outer block: {}", shadowed_binding);
        crate::line();
    }
}

mod declare_first {
    pub fn main() {
        crate::title("Declare first");

        let a_binding;
        {
            let x = 2;
            a_binding = x * x;
        }

        println!("a binding: {}", a_binding);

        let another_binding;

        // Can't do this, not bound yet
        // println!("another binding: {}", another_binding);

        another_binding = 1;
        println!("another binding: {}", another_binding);

        crate::line();
    }
}

mod freezing {
    pub fn main() {
        crate::title("freezing");

        let mut _mutable_integer = 7i32;

        {
            let _mutable_integer = _mutable_integer;

            // Can't assign it, it's frozen in the scope
            // _mutable_integer = 50;
        }

        _mutable_integer = 5; // we can do this now

        crate::line();
    }
}

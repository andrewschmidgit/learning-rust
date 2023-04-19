use std::{
    error::Error,
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file_result = File::open("hello.txt");

    // That's a lot of matches.. Could start a fire
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // Let's try again: with unwrapping!
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt")
                .unwrap_or_else(|error| panic!("Problem creating the file: {:?}", error))
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });

    // Will auto-panic
    let greeting_file = File::open("hello.txt").unwrap();

    // Panicking with custom message
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in the project");

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    // The error will be returned at the ?
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// None will be automatically returned at the point of ?
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

pub struct Guess {
    value: i32,
}

// By making value private, we can guarantee validation
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("value must be between 1 and 100")
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

use std::{
    error::Error,
    fs::{File, self},
    io::{self, ErrorKind, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    let name = "hello.txt";

    let _file = get_and_create_file_panic(name);
    let _file = get_and_create_file_closure(name);
    let _file = get_and_create_file_unwrap(name);
    let _file = get_and_create_file_expect(name);

    let _username = read_username_from_file(name);
    let _username = read_username_from_file_shorter(name);
    let _username = read_username_from_file_shortest(name);

    Ok(())
}

fn get_and_create_file_panic(name: &str) -> File {
    let result = File::open(name);

    match result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(name) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    }
}

fn get_and_create_file_closure(name: &str) -> File {
    File::open(name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(name).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    })
}

fn get_and_create_file_unwrap(name: &str) -> File {
    File::open(name).unwrap()
}

fn get_and_create_file_expect(name: &str) -> File {
    File::open(name).expect("hello.txt should be included")
}

fn read_username_from_file(name: &str) -> Result<String, io::Error> {
    let username_file_result = File::open(name);

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

fn read_username_from_file_shorter(name: &str) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(name)?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shortest(name: &str) -> Result<String, io::Error> {
    fs::read_to_string(name)
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

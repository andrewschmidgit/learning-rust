fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is now invalid, s2 owns "hello"

    let s3 = s2.clone(); // s2 and s3 are both valid, and have deep copies

    // Ownership got passed to the function, so s3 can't be used after this point
    let cased = get_first_character_capitalized(s3);
    println!("h capitalized: {cased}");

    let s4 = String::from("hello again");
    let cased = get_first_character_capitalized_reference(&s4);
    println!("s4: {s4}, cased: {cased}");
    
    let mut s5 = String::from("help, I'm lowercase");
    uppercase_first_letter(&mut s5);
    println!("{s5}");

    let mut s6 = String::from("hello world");
    let word_index = bad_get_first_word(&s6);
    
    let word = good_get_first_word(&s6);
    let word = great_get_first_word(&s6);

    println!("string: {}, word: {}", s6, word);

}                         

fn get_first_character_capitalized(s: String) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>()
    }
}

fn get_first_character_capitalized_reference(s: &String) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>()
    }
}

fn uppercase_first_letter(s: &mut String) {
    let c = s.remove(0).to_string();
    let c = c.to_uppercase();
    s.insert_str(0, c.as_str());
}

// Bad because the index isn't tied to the string. the string could change
fn bad_get_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn good_get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
fn great_get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

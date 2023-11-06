pub fn lifetimes_main() {
    let one = "test";
    let two = "test but longer";
    let longer = longest(one, two);

    println!("one: {}, two: {}, longer of the two: {}", one, two, longer);

    let one = "scoped test";
    {
        let two = "super duper scoped test";
        let longer = longest(one, two);
        println!("one: {}, two: {}, longer of the two: {}", one, two, longer);
    }

    // Lifetimes work
    // --------------
    // let one = "bad test";
    // let result;
    // {
    //     let two = String::from("bad test long boi");
    //     result = longest(one, two.as_str());
    // }
    //
    // println!("one: {}, two: {}, longer of the two: {}", one, two, result);
    
    let novel = String::from("A killer intro. Blah blah book stuff...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let _i = ImportantExcerpt {
        part: first_sentence // novel lasts longer than this, so the lifetime is valid
    };
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

pub fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    get_element_by_index(&v, 2);
    iterate_over(&v);
    double_each(&mut v);

    spreadsheet::make_row();
}

fn get_element_by_index(list: &Vec<i32>, index: usize) {
    let third = &list[index];
    println!("The third element is {third}");

    let third = list.get(index);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

fn iterate_over(list: &Vec<i32>) {
    for element in list {
        print!("{element} ");
    }
    println!("");
}

fn double_each(list: &mut Vec<i32>) {
    for i in &mut *list {
        *i *= 2;
    }
}

mod spreadsheet {
    pub enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    pub fn make_row() -> Vec<Cell> {
        vec![
            Cell::Int(3),
            Cell::Float(10.12),
            Cell::Text(String::from("blue")),
        ]
    }
}

pub fn generic_main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let number = largest(&number_list);

    println!("The largest number is {}", number);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let char = largest(&char_list);

    println!("The largest char is {}", char);

    let integer = Point { x: 5, y: 10 };
    println!("integer point: {:?} (x: {}, y: {})", integer, integer.x(), integer.y);
    let float = Point { x: 1.0, y: 4.0 };
    println!("float point: {:?} (x: {}, y: {})", float, float.x(), float.y);
    // let mixed = Point { x: 1, y: 4.0 }; DOESN'T WORK
    let mixed = MixedPoint { x: 1, y: 4.0 };
    println!("mixed point: {:?} (x: {}, y: {})", mixed, mixed.x, mixed.y);

    let distance_from_origin = float.distance_from_origin();
    println!("point: {:?}, distance from origin: {}", float, distance_from_origin);
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

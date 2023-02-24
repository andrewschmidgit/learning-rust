fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("test@example.com"),
        ..user1
    };

    // can't use user1 anymore, because it has a string username
    
    let width1 = 30;
    let height1 = 50;
    let rectangle = Rectangle {
        width: width1,
        height: height1
    };
    println!("The area of the rectangle is {} square pixels", area(width1, height1));
    println!("The area of the rectangle is {} square pixels", tuple_area((width1, height1)));
    println!("The area of the rectangle is {} square pixels", rectangle_area(&rectangle));

    println!("Rectangle: {:?}", rectangle);
    println!("Rectangle: {:#?}", rectangle);

    // takes ownership
    let rectangle = dbg!(rectangle);
    println!("The area of the rectangle is {} square pixels", rectangle.area());

    let small = Rectangle {
        width: 30,
        height: 20
    };

    let medium = Rectangle {
        width: 50,
        height: 50
    };
    let large = Rectangle {
        width: 500,
        height: 51
    };

    println!("Can {:?} fit into {:?}? '{}'", small, medium, medium.can_hold(&small));

    let square = Rectangle::square(2);

    println!("Made a square: {:?}", square);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn rectangle_area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Rectangle { width: size, height: size }
    }
}

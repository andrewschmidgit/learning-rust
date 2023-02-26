fn main() {
    println!("Hello, world!");
    ips();
    messages();

    let quarter_value = value_in_cents(Coin::Quarter(UsState::Alabama));
    let dime_value = value_in_cents(Coin::Dime);
    let nickel = value_in_cents(Coin::Nickel);
    let penny = value_in_cents(Coin::Penny);

    println!("Quarter: {quarter_value}");
    println!("Dime: {dime_value}");
    println!("Nickel: {nickel}");
    println!("Penny: {penny}");
}

fn ips() {
   let four = IpKind::V4(127, 0, 0, 1);
   let six = IpKind::V6(String::from("127::0::0::1")); 

   println!("ipv4: {:?}", four);
   println!("ipv6: {:?}", six);
}

#[derive(Debug)]
enum IpKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn messages() {
    let quit = Message::Quit;
    let r#move = Message::Move { x: 30, y: -2 };
    let write = Message::Write(String::from("wow"));
    let color = Message::ChangeColor(255, 255, 255);

    println!("{:?}", quit);
    println!("{:?}", r#move);
    println!("{:?}", write);
    println!("{:?}", color);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("Wow crazy look at the message {:?}", self);
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => i,
        None => None,
    }
}

fn if_let() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // This is the same
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max),
    }
}

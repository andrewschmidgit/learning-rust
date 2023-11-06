use generics::generic_main;
use traits::traits_main;
use lifetimes::lifetimes_main;

pub mod generics;
pub mod traits;
pub mod lifetimes;

fn main() {
    println!();
    println!("------------------------------------------------");
    println!();
    generic_main();
    println!();
    println!("------------------------------------------------");
    println!();
    traits_main();
    println!();
    println!("------------------------------------------------");
    println!();
    lifetimes_main();
}

use generics::generic_main;
use traits::traits_main;

pub mod generics;
pub mod traits;

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
}

// This brings Asparagus into scope
use crate::garden::vegetables::Asparagus;

// This exposes and includes the garden module, either garden.rs or garden/mod.rs
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing a {:?}!", plant);
}

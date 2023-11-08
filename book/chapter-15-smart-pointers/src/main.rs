pub mod boxes;
pub mod deref;
pub mod drop;
pub mod rc;

fn main() {
    println!("\nBoxes:");
    boxes::run();
    println!("\nDeref:");
    deref::run();
    println!("\nDrop:");
    drop::run();
    println!("\nReference Counting:");
    rc::run();
}

mod basic;
mod messages;
mod shared_state;

fn main() {
    println!("\nBasic threads");
    basic::run();

    println!("\nMessage passing");
    messages::run();

    println!("\nShared state");
    shared_state::run();
}

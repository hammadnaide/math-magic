use std::num::Wrapping;

fn main() {
    let a = Wrapping(10);
    let b = Wrapping(20);

    println!("a + b = {}", a + b);
}

use std::cmp::Ordering;

fn main() {
    let mut x = 10;
    let y = 20;

    match x.cmp(&y) {
        Ordering::Less => println!("{} is less than {}", x, y),
        Ordering::Greater => println!("{} is greater than {}", x, y),
        Ordering::Equal => println!("{} equals {}", x, y),
    }
}

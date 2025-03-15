use std::ops::Add;

fn main() {
    let numbers = vec![1, 2, 3, 4];
    let sum = numbers.into_iter().sum();
    println!("The sum of the numbers is {}", sum);
}

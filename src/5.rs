  
fn main() {
    let a = 5;
    let b = 2;
    println!("The sum of {} and {} is {}", a, b, add(a, b));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
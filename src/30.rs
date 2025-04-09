fn main() {
    // Randomly generate some numbers
    let numbers: Vec<isize> = [10, 20, 30].sample_iter().take(5).collect();
    
    println!("The five random numbers are: {:?}", numbers);
}

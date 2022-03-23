use std::io;
use std::{f64};

fn main() {
    println!("Enter the number of the radius: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read entered data");

    let radius:f64 = input.trim().parse().expect("Program only processes numbers, Enter number");

    let n:f64 = 3.14159;

    let area:f64 = n * radius.powi(2);

    println!("A={:.4}", area);
}

use std::io;
use std::{i8};

fn main() {
    println!("Input First number ? ");
    let mut var1 = String::new();
    io::stdin().read_line(&mut var1).expect("Unable to read entered data");

    println!("Input second number ? ");
    let mut var2 = String::new();
    io::stdin().read_line(&mut var2).expect("Unable to read entered data");

    let a:i8 = var1.trim().parse().expect("Program only processes numbers, Enter number");
    let b:i8 = var2.trim().parse().expect("Program only processes numbers, Enter number");

    let x:i8 = a + b;

    println!("{} + {} = {}", a, b, x);
}

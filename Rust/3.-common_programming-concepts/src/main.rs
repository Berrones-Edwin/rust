// formula C to F
// F = ( C * 1.8) + 32
// C = (F-32) / 1.8
use std::io;
fn main() {
    println!("Celsius to Fahrenheit and viceverse");

    let mut option = String::new();

    println!("Select an option");
    println!("Option 1: Celsius to Fahrenheit");
    println!("Option 2: Fahrenheit to Celsius");

     io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");
use std::io;

fn main() {
    println!("1 = Convert Celsius to Fahrenheit ");
    println!("2 = Convert Fahrenheit  to Celsius");

    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    let value: u32 = value.trim().parse().expect("Please type a number");

    if value == 1 {
        celsius_to_fahrenheit();
    } else {
        fahrenheit_to_celsius();
    }
}

fn enter_numbers_to_convert() -> u32 {
    let mut grades = String::new();

    io::stdin()
        .read_line(&mut grades)
        .expect("Failed to read line");

    let grades: u32 = grades.trim().parse().expect("Please type a number");

    grades
}
fn celsius_to_fahrenheit() {
    println!("Enter grades to convert Celsius to Fahrenheit ");
    let grades_number = enter_numbers_to_convert();
    let result = (grades_number * 9 / 5) + 32;

    println!("The results is: {}", result);
}

fn fahrenheit_to_celsius() {
    println!("Enter grades to convert Fahrenheit  to Celsius");
    let grades_number = enter_numbers_to_convert();
    let result = (grades_number - 32) * 5 / 9;

    println!("The results is: {}", result);
}

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!!");

    //  Other ways for specific parameters and I generate random number of 1 to 100
    //   is using 1..=100
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is {}", secret_number);
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        println!("Please input your guess");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!!"),
            Ordering::Greater => println!("To big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}

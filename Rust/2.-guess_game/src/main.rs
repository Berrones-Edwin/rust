use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[derive(Debug)]
pub struct Guess {
    value: u32,
}
impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Gues Value muts be between 1 and 100, got: {}", value);
        }

        Guess { value }
    }
    pub fn value(&self) -> u32 {
        self.value
    }
}
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
        let guess = Guess::new(guess);

        println!("You guessed: {:?}", guess);

        println!("Please input your guess");
        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("To small!!"),
            Ordering::Greater => println!("To big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}

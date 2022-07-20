#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn add_two_to_two() {
        assert_eq!(4, add_two(2));
    }
    #[test]
    fn add_two_to_six() {
        assert_eq!(8, add_two(6));
    }

    #[test]

    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore = "function to test"]
    fn expensive_test() {
        assert_eq!(100, add_two(98));
    }

    #[test]

    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two doesn't equal four"))
        }
    }
}

pub fn add_two(num: i32) -> i32 {
    println!("The number is {}", num);
    num + 2
}

// fn prints_and_returns_10(a: i32) -> i32 {
//     println!("I got the value {}", a);
//     10
// }

fn main() {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn exploring() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn another_text() {
        panic!("This test will fail :s");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };

        let smaller = Rectangle {
            length: 5,
            width: 6,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };

        let smaller = Rectangle {
            length: 5,
            width: 6,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_add_two() {
        assert_eq!(5, add_two(2));
    }

    #[test]
    fn greetings_contains_name() {
        let result = greetings("Carol");

        assert!(
            result.contains("Carol"),
            "Greeting didn't contain name, value was {}",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(250);
    }
}

pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.length
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greetings(name: &str) -> String {
    format!("Hello {}", name)
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must less than or equal to 100, got {}", value);
        } else if value > 100 {
            panic!("Guess value must greater than or equal to 1, got {}", value);
        }
        // if value < 1 || value > 100 {
        //     panic!("Gues value must be between 1 and 100, got {}", value);
        // }

        Guess { value }
    }
}
// fn main() {}

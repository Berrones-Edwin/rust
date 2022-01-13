use std::fmt;

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// impl fmt::Display for Point2D {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Customize so only `x` and `y` are denoted.
//         write!(f, "x: {}, y: {}", self.x, self.y)
//     }
// }

fn main() {
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {}", count);

    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining  = {}", remaining);

    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {}", count);

    // let mut counter = 0;

    // let result = loop {
    //     counter +=1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("The results is {}",result);

    // let mut number = 3;

    // while number != 0 {
    //     println!("{}!", number);

    //     number -= 1;
    // }
    // println!("Finish");

    // let a  = [10,20,30,40,50];

    // let mut index = 0;
    // while index < 5 {
    //     println!("The results is {}",a[index]);
    //     index+=1;
    // }

    // let a  = [10,20,30,40,50];

    // for element in a{
    //     println!("The value is {}",element);
    // }

    // for number in (1..4).rev() {
    //     println!("The value is {}", number);
    // }
}

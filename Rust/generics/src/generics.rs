// fn find_the_largest_number(list_numbers: &Vec<i32>) -> i32 {
//     let mut largest = list_numbers[0];

//     for &number in list_numbers {
//         if number > largest {
//             largest = number
//         }
//     }

//     largest
// }

// fn largest_i32(list: &[i32]) -> i32 {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> char {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item
//         }
//     }
//     largest
// }

// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item
//         }
//     }
//     largest
// }

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
// struct Point<T> {
//     x: T,
//     y: T,
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
// impl Point<f64> {
//     fn distance_from_origin(&self) -> f64 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }
fn main() {
    // let my_list = vec![34, 55, 98, 57, 65, 100];
    // let my_list1 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    // let largest = find_the_largest_number(&my_list);
    // let largest = largest(&my_list);
    // let largest1 = find_the_largest_number(&my_list1);

    // println!("The largest number is {}", largest);
    // println!("The largest number is {}", largest1);
    // assert_eq!(largest, 100);
    // assert_eq!(largest1, 6000);

    // let point_x = Point {
    //     x: 20,
    //     y: String::from("Hello world"),
    // };
    // let point_y = Point { x: 1.2, y: 55.5 };

    // println!("{:?}", point_y.x());
    // println!("{:?}", point_y.distance_from_origin());

    let p1 = Point { x: 5, y: 10.4 };

    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    print!("p3.x = {} p3.y = {}", p3.x, p3.y);
}

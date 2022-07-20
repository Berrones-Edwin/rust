// use std::fmt::Display;

// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Pair { x, y }
//     }
// }

// // Todos los que implementen Display pueden usar ToString
// // , la biblioteca estándar implementa el trait ToString en cualquier tipo que implemente el trait Display
// // impl<T: Display> ToString for T {
// //     // --snip--
// // }

// // Solo para los que implementen display y partialord
// //pueden usar el metodo cmp_display
// impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("El largest number is x= {}", self.x);
//         } else {
//             println!("El largest number is y= {}", self.y);
//         }
//     }
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

use std::fmt::Display;

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announce: &str) -> &str {
        println!("Attention please:{}", announce);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);

    if x.len() > y.len() {
       return  x
    }
    {
        y
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");

    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{:?}", i);

    let s: &'static str = "I have a static lifetime";
    // let x = 5;
    // // {
    // //     r= &x;
    // // }
    // let r = &x;

    // println!("{}", r);

    // let str1 = String::from("Hello world");
    // let result;
    // {
    //     let str2 = String::from("Hello World!!");
    //     result = longest(str1.as_str(), &str2);
    // }
    // println!("{}", result);
}

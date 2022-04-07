
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
fn main() {
    // let x = value_in_cents(Coin::Quarter(UsState::Alaska));
    // print!("{}", x);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?} - {:?}", six, none);

    let some_u8_value = 0u8;
    match some_u8_value {
        1=>println!("one"),
        3=>println!("three"),
        5=>println!("five"),
        7=>println!("seven"),
        _=>println!("other number {}",some_u8_value)
    }
}

// fn value_in_cents(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quearte from {:?}", state);
//             25
//         }
//     }
// }

fn plus_one(x: Option<i32>) -> Option<i32> {
    println!("{:?}",x);
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

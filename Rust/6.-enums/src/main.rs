#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    let some_value = Some(35);

    if let Some(3) = some_value {
        println!("three");
    }

    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;

    match &coin {
        Coin::Quarter(state) => println!("State Quarter {:?}", state),
        _ => count = count + 1,
    }

    if let Coin::Quarter(_state) = coin {
        println!("State Quarter {:?}", _state);
    } else {
        count = count + 1
    }
}

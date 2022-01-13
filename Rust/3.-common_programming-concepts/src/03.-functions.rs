/**
 * Rust code uses snake case as the conventional style for function and vairbales names. In snake case, all letters are lowercase and underscores seprate words.
*/

fn main() {
    // let x= 9; // Not return value
    let y = {
        let x = 3;
        x + 1
    };

    let x = five();
    let plus_one_var = plus_one(5);

    println!("Hello world");
    println!("The value of Y is , {}", y);
    println!("The value of X is , {}", x);
    println!("The value of plus_one_var is , {}", plus_one_var);

    another_function();
    another_function_with_parameters(35);
    another_function_with_2_parameters(35, 'H')
}

fn five() -> i32 {
    5
}
fn plus_one(x:i32)->i32{
    x+1
}

fn another_function() {
    println!("another function");
}

/**
 * In function signatures, you must declare the type of each parameter. This is a deliberate decision in  Rust's design: requiring type annotations in function definitions.
*/
fn another_function_with_parameters(x: i32) {
    println!("another function with parameters, {}", x);
}
fn another_function_with_2_parameters(x: i32, z: char) {
    println!("another function with parameters, {} - {}", x, z);
}

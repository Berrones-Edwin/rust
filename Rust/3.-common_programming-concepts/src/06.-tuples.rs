fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);
use std::io;
fn main() {
    // A tuple with a bunch of different types
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 1u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuples of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);

    println!("pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));

    println!("one element tuple {:?}", (5u32));
    println!("just and integer {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;

    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    let a = [0, 1, 2, 3, 4, 5];
    println!("Please enter an array index");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a  number");

    let element = a[index];

    println!("The values of element at index {} is {}", index, element);
}

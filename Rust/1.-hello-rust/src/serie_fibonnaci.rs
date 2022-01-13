fn main() {
    let mut number_initial = 0;
    let mut number_second = 1;
    let mut result = 0;

    while result < 100 {
        result = number_initial + number_second;

        println!("{}",result);
        // println!("{} + {} = {}", number_initial, number_second, result);

        number_initial = number_second;
        number_second = result;
    }
}

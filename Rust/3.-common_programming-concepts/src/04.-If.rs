fn main() {
    // let number = 55;
    // if number > 10 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }

    // let number = 88;

    //Esto esta mal, RUST espera una expresion boolean
    //Rust no convertira a boolean la expresion
    // if number {
    //     println!("number is true");
    // }
    // if number != 0 {
    //     println!("number was something other than zero");
    // }

    /*
     * Rust arrojara el primer resultado que se cumpla
     * y los demas los ignorara.
     * Con el primer bloque que se cumpla ahi termina la ejecucion
     */
    // let number = 6;

    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }

    let condition = true;

    let number = if condition { 5 } else { 6 };

    // En esta linea nos devolvera un error
    // Ya que rust espera que los valores de retorno sean
    // del mismo tipo
    // let number = if condition { 5 } else { "six" };

    println!("Result of the variable {}",number);
}

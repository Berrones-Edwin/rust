fn main() {
    /**
    * We have two types data scalar and compounds
    * scalar
    *
    * signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
       unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
       floating point: f32, f64
       char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
       bool either true or false
       and the unit type (), whose only possible value is an empty tuple: ()


       Compound Types
       arrays like [1, 2, 3]
       tuples like (1, true)
    */
    let numberSigned: i32 = 56;
    let numberUnsigned: u32 = -56;

    /**
     * Floating types point
     * There are two types data 32 and 64 bits
     * for default is 64 bits.
     */
    let numberFloating = 63.23; //f64
    let numberFloating32: f32 = 36.25;

    // NUMERIC OPERATIONS
    /**
     * In Rust we have all types of oprations
     * add, substraction, division, multiplication, remainder
     * Note: Intenger division rounds down to the nearest integer
     */
    // addition
    let sum = 5 + 10;

    // substraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // Division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; //Results in 0

    // Remainder
    let remainder = 43 % 5;

    /**
     * BOOLEAN TYPE
     * Only can save one value (true | false)
     */
    let t = true;
    let f: bool = false;

    /**
     * CHARACTER TYPE
     */
    let c = 'z';
    let z = 'C';

    /**
     * Coumpons types can group multiple values into one type
     */
    // TUPLE TYPE
    // Tuple can save multiple values into one  coumpound type.
    // The length cannot grow or shrink in size
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Desctructure tuple
    let (x, y, z) = tup;
    println!("The value of Y is: {}", y);

    // Access values of tuple using period (.)
    let five_hundred = tup.0;
    let six_point_four = tup.1;

    // ARRAYS
    /**
     * In Arrays every element of an array must have the same type. Arrays in Rust are different form arrays in some other languages because in Rust have a fixed length, like Tuples
     */
    let a = [1, 2, 3, 4, 5, 6];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    /**
     * You would write an array's type using square brackets , and withon the brackets include the type of each element, a semicolon, and then the number of elements in the arrya, like so:
    */
    let a:[i32,5]=[1,2,3,4,5];

    /**
     * The array named b will contain 5 elements that will all be seth to the value 3 initially. this is the smae as writing let a =[3,3,3,3,3]; but in a more concise way;
     * 
    */
    let b =[3;5];

    let firstMonth = months[0];
}

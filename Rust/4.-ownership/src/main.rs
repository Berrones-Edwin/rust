// Propiedades y funciones

// fn main() {
//     let s = String::from("hello");

//     takes_ownerships(s);

//     let x = 30;
//     makes_copy(x);
//     println!("fn main {}", x);
// }
// fn takes_ownerships(some_string: String) {
//     println!("{}", some_string);
// }
// fn makes_copy(some_int: i32) {
//     println!("{}", some_int);
// }

// Valores retorno
// fn main() {
//     let s1 = gives_ownership();
//     println!("{}", s1);

//     let s2 = String::from("hello");

//     let s3 = takes_and_gives_ownership(s2);
//     // println!("{}", s2);
//     println!("{}", s3);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("hello");
//     some_string
// }

// fn takes_and_gives_ownership(some_string:String)->String{
//     some_string
// }

// fn main(){
//     let s1 = String::from("hello");
//     let (s2,s2_len)  = calculate_length(s1);

//     println!("th length of {} is {}",s2,s2_len);
//     // println!("{}",s1);

// }

// fn calculate_length(s:String)->(String,usize){
//     let length = s.len();

//     (s,length)
// }

// REFERENCIAS Y PRESTAMOS
// fn main() {
//     let s1 = String::from("Hello");
//     let length = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, length);

//     let mut s2 = String::from("Hello");
//     let string_change = change(&mut s2);
//     println!("{:?}", string_change);
//     println!("{:?}", s2);

// This is wrong
// let mut s3= String::from("Hello");
// let r2 = &mut s3;
// let r3 = &mut s3;
// println!("{} - {}",r2,r3);

// This is OK
// let mut s3 = String::from("Hello");
// {
//     let r2 = &mut s3;
// }
// let r3 = &mut s3;

// This is wrong
// let mut s3 = String::from("Hello");
// let r1 = &s3;
// let r2 = &s3;
// let r3 = &mut s3;

// REFERENCIAS COLGANTES
// }
// This is wrong
// fn dangle() -> &String {
//     let s=String::from("Hello");
//     &s
// }
// fn dangle() -> String {
//     let s=String::from("Hello");
//     s
// }
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn change(s: &mut String) {
//     s.push_str(" ,world");
// }

// SLICES
fn main() {
    let mut s = String::from("Hello");
    let word = first_word(&s);

    println!("{}", word);
    
    s.clear();
    
    println!("{}", s);
    // println!("{}", word);

    // let hello_world = String::from("Hello world");

    // let hello = &hello_world[0..5];
    // let world = &hello_world[5..11];

    // println!("helo {} world {}",hello,world);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    println!("{:?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return i;
            return &s[0..i];
        }
    }
    &s[..]
}

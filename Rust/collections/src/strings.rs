
fn main() {
    // Get the same result
    let mut s = String::from("Hello");

    let mut x = "hello".to_string();

    println!("{} {}", s, x);

    let s2 = "bar";

    s.push_str(s2);

    println!("{} {}", s, s2);

    x.push('o');
    println!("{} ", x);
}

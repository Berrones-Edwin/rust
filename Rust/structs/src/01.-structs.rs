
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("Edwin"),
        email: String::from("edwin@gmail.com"),
        sign_in_count: 4,
    };

    user1.email = String::from("Othreremail@gmail.com");

    let user2 = User {
        email: String::from("myemail@gmail.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{:#?}",user2);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 2,
        active: false,
    }
}

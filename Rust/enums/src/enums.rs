#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
enum IpAddrKind1 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn show(&self) {
        println!("Hola mundo");
    }
}

// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
fn main() {
    // let four = route(IpAddrKind::V4);
    // let six = IpAddrKind::V6;

    // println!("{:?} ' {:?}", four, six);

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // println!("{:?}", home);

    // let home = IpAddrKind1::V4(127, 0, 0, 1);
    // let loopback = IpAddrKind1::V6(String::from("::1"));

    // let m = Message::Write(String::from("Hola mundo write enum"));
    // m.show();

    // let some_string = Some(5);

    // let some_number = Some("Hola mundo");

    // let absent_number:Option<i32> = None;

    // let x = 8;
    // let y = Some(5);

    // let r = x * y;
}

// fn route(ip_type: IpAddrKind) -> IpAddrKind {
//     ip_type
// }



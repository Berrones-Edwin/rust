#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    // let w = 10;
    // let h = 50;

    // let rec1 = (10, 50);
    let rec1 = Rectangle {
        width: 10,
        height: 50,
    };
    let rec2 = Rectangle {
        width: 5,
        height: 10,
    };
    let rec3 = Rectangle {
        width: 200,
        height: 500,
    };

    // println!("The area is {:?}", rec1);
    // println!("The area is {:#?}", rec1);
    println!("The area is {}", rec1.area());
    println!("Can rec1 hold rec2, {}", rec1.can_hold(&rec2));
    println!("Can rec1 hold rec3, {}", rec1.can_hold(&rec3));
    
    let rec4 =Rectangle::square(4);
    println!("The area is {:#?}", rec4);
}

// fn calculate_area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }
// fn calculate_area(rec: &Rectangle) -> u32 {
//     rec.width * rec.height
// }
// fn calculate_area(width: u32, height: u32) -> u32 {
//     width * height
// }

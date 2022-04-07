extern crate communicator;
enum TrafficLight {
    Red,Yellow, Green
}
use TrafficLight::*;
// use TrafficLight::{Red,Yellow};

// pub mod a {
//     pub mod series {
//         pub mod of {
//             pub fn nested_function(){
//                 println!("Hello world");
//             }
//         }
//     }
// }
// use a::series::of::nested_function;

// use a::series::of;
fn main() {
    // nested_function();
    communicator::client::connect();
    let red = Red;
    let yellow = Yellow;

    let green  = Green;
    // let green  = TrafficLight::Green;

}

#[derive(Debug)]
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String, //field private
}
pub enum Appetizer {
    Soup,
    Salad,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}
fn fix_incorrect_order() {
    cook_order();
    super::serve_order();
}

fn cook_order() {}
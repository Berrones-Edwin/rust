#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod front_of_house;
mod back_of_house;

fn serve_order() {}


use front_of_house::hosting::add_to_wishlist;
use back_of_house::{Appetizer,Breakfast};

pub fn eat_at_restaurant() {
    // Absolute
    add_to_wishlist();
    // Relative
    add_to_wishlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {:?} toast please", meal);

    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;
}

mod external_module;

pub use crate::external_module::external_dependence;

mod front_of_house {
  pub mod hosting {
    pub fn add_to_wailist() {}

    fn seat_at_table() {}
  }

  pub mod serving {
    fn take_order() {}

    pub fn serve_order() {}

    fn take_payment() {}
  }
}

mod back_of_house {
  pub enum Appetizer {
    Soup,
    Salad,
  }

  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
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
    super::front_of_house::serving::serve_order();
  }

  fn cook_order() {}
}

pub fn eat_at_restaurant() {
  let mut meal = back_of_house::Breakfast::summer("Rye");
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);

  let order1 = back_of_house::Appetizer::Soup;
  let order2 = back_of_house::Appetizer::Salad;

  external_dependence::some_method();


  // absolute path
  crate::front_of_house::hosting::add_to_wailist();

  // relative path
  front_of_house::hosting::add_to_wailist();
}

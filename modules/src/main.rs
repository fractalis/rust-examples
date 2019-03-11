use std::{cmp::Ordering};
use std::io::{self, Write};

mod sound {
    pub mod instrument {
        pub fn clarinet() {
            super::breathe_in();
        }
    }

    fn breathe_in() {
    }
}

mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1
            }
        }
    }
}

mod menu {
    pub enum Appetizer {
        Soup,
        Salad
    }
}

use crate::sound::instrument;

fn main() {

    crate::sound::instrument::clarinet();

    sound::instrument::clarinet();

    instrument::clarinet();

    let mut v = plant::Vegetable::new("Squash");

    v.name = String::from("Butternut Squash");
    println!("{} are delicious", v.name);

    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;
}

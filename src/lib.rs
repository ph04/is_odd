extern crate is_odd_macro;

use is_odd_macro::is_odd_builder;

pub fn is_odd(number: usize) -> bool {
    is_odd_builder!()
}

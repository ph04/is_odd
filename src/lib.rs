extern crate is_odd_macro;

use is_odd_macro::is_odd_builder;

/// Returns `true` if the given number is odd.
pub fn is_odd(number: usize) -> bool {
    is_odd_builder!()
}

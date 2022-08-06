extern crate proc_macro;

use proc_macro::TokenStream;

const ARROW: &str = "=>";
const TRUE: &str = "true";
const FALSE: &str = "false";

#[proc_macro]
pub fn is_odd_builder(_: TokenStream) -> TokenStream {
    let mut result = String::from("match number{");

    (0..usize::MAX)
        .for_each(|number| {
            result.push_str(&number.to_string());

            result.push_str(ARROW);

            result.push_str(if number & 1 == 1 { TRUE } else { FALSE });

            result.push(',');
        });

    result.push_str("_=>unreachable!()}");

    result.parse().unwrap()
}

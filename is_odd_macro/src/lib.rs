extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn is_odd_builder(_: TokenStream) -> TokenStream {
    let mut result = String::from("match number {");

    (0..usize::MAX)
        .for_each(|number| {
            let mut statement = number.to_string();

            statement.push_str("=> ");

            statement.push_str(&(number & 1).to_string());
        });

    result.push_str("_ => unreachable!() }");

    result.parse().unwrap()
}

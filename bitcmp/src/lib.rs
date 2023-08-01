use proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::parse_macro_input;


#[proc_macro]
pub fn bitcmp(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    TokenStream::from(expanded)
}

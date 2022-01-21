use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer(_: TokenStream) -> TokenStream {
    "fn answer(a:u32) -> u32 {a}".parse().unwrap()
}

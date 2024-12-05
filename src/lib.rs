use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn stem(_: TokenStream) -> TokenStream {
    TokenStream::from(quote!(std::path::Path::new(file!())
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap()
        .to_string()))
}

mod meta;
use proc_macro::TokenStream;

#[proc_macro_derive(ModuleDBMeta, attributes(meta))]
pub fn derive_db_meta(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    match meta::derive_db_meta_inner(&input) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

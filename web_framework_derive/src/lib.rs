extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn controller_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let func_name = &input.sig.ident;

    let gen = quote! {
        use std::collections::HashMap;

        pub fn controller_fn_list() -> HashMap<String, fn()> {
            let mut map: HashMap<String, fn()> = HashMap::new();
            map.insert(String::from(stringify!(#func_name)), #func_name as fn());
            map
        }

        #input
    };

    gen.into()
}

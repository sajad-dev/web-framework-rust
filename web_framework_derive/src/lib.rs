extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn controller_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let func_name = &input.sig.ident;

    let mut attr_iter = _attr.into_iter();

    let mut name = String::from(stringify!(#func_name));

    if let Some(proc_macro::TokenTree::Literal(lit)) = attr_iter.next() {
        name = lit.to_string()
    }

    let gen = quote! {
        use std::collections::HashMap;

        type fn_type = fn(http_request:HashMap<String, String>) -> String;
        pub fn controller_fn_hashmap() -> HashMap<String, fn_type> {
            let mut map: HashMap<String, fn_type> = HashMap::new();
            map.insert(#name.trim_matches('"').to_string(), #func_name as fn_type);
            map
        }

        #input
    };

    gen.into()
}

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Span, Ident};
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
    
    let unique_name = Ident::new(&format!("controller_fn_hashmap_{}", name.trim_matches('"').to_string()), Span::call_site());


    let gen = quote! {

        pub fn #unique_name() -> HashMap<String, fn(http_request:HashMap<String, String>) -> String> {
            let mut map: HashMap<String, fn(http_request:HashMap<String, String>) -> String> = HashMap::new();
            map.insert(#name.trim_matches('"').to_string(), #func_name as fn(http_request:HashMap<String, String>) -> String);
            map
        }

        #input
    };

    gen.into()
}

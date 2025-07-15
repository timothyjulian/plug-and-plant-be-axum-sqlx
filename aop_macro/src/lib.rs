use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn log_calls(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let vis = &input.vis;
    let sig = &input.sig;
    let fn_name = &sig.ident;
    let inputs = &sig.inputs;
    let block = &input.block;
    let asyncness = &sig.asyncness;

    // Extract argument names as idents to log
    let arg_idents: Vec<_> = inputs.iter().filter_map(|arg| {
        match arg {
            syn::FnArg::Typed(pat) => Some(&pat.pat),
            _ => None,
        }
    }).collect();

    let expanded = quote! {
        #vis #sig {
            println!("--> Entering {} with args: {:?}", stringify!(#fn_name), (#(&#arg_idents),*));
            let result = (|| #asyncness #block)().await;
            println!("<-- Exiting {} with result: {:?}", stringify!(#fn_name), result);
            result

        }
    };

    expanded.into()
}
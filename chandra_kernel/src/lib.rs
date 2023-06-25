#![recursion_limit = "128"]

mod function;
mod kernel;
mod parseatt;
mod parse_function;
mod parse_cpu_function;

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::{proc_macro_error};
use syn::{parse_macro_input, DeriveInput, Data};


#[proc_macro_error]
#[proc_macro_attribute]
pub fn kernel(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    kernel::kernel(attr, tokens, quote::quote!(::chandra))
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn ChandraFunction(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    function::function(attr, tokens, quote::quote!(::chandra), false)
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn ChandraExtension(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    function::function(attr, tokens, quote::quote!(::chandra), true)
}

#[proc_macro_error]
#[proc_macro_derive(ChandraStruct, attributes(For, Index, XPar))]
pub fn chandra_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let attrs = &input.attrs;
    //println!("{:#?}", attrs);

    if let Data::Struct(s) = input.data {
        for field in s.fields {
            //println!("{:?}: {:#?}", field.ident, field.attrs);
        }
    }
    
    quote::quote!().into()
}

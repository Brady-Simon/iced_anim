extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Derive macro generating an impl of the trait `Animate`.
#[proc_macro_derive(Animate)]
pub fn animate_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the identifier and data from the input
    let name = input.ident;
    let data = input.data;

    let Data::Struct(data_struct) = data else {
        panic!("Animate can only be derived for structs");
    };

    let Fields::Named(fields) = data_struct.fields else {
        panic!("Animate can only be derived for structs with named fields");
    };

    // Generate the implementation based on the struct's fields
    // Generate code for each field that implements Animate
    let fields_animate = fields.named.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            #name: <#ty as ::iced_animation_builder::Animate>::animate(&start.#name, &end.#name, progress, curve),
        }
    });

    let impl_gen = quote! {
        impl ::iced_animation_builder::Animate for #name {
            fn animate(start: &Self, end: &Self, progress: f32, curve: ::iced_animation_builder::Curve) -> Self {
                Self {
                    #(#fields_animate)*
                }
            }
        }
    };

    // Convert the generated implementation into a TokenStream and return it
    TokenStream::from(impl_gen)
}

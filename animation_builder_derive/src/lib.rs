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

    // TODO: Support other types of data structures
    let Data::Struct(data_struct) = data else {
        panic!("Animate can only be derived for structs");
    };

    let Fields::Named(fields) = data_struct.fields else {
        panic!("Animate can only be derived for structs with named fields");
    };

    let component_fields = fields.named.iter().map(|f| {
        let ty = &f.ty;
        quote! {
            total += <#ty as ::iced_anim::Animate>::components();
        }
    });

    let update_fields = fields.named.iter().map(|f| {
        let name = &f.ident;
        quote! {
            ::iced_anim::Animate::update(&mut self.#name, components);
        }
    });

    let distance_fields = fields.named.iter().map(|f| {
        let name = &f.ident;
        quote! {
            distances.push(::iced_anim::Animate::distance_to(&self.#name, &end.#name));
        }
    });

    let impl_gen = quote! {
        impl ::iced_anim::Animate for #name {
            fn components() -> usize {
                let mut total = 0;
                #(#component_fields)*
                total
            }

            fn update(&mut self, components: &mut impl Iterator<Item = ::core::primitive::f32>) {
                #(#update_fields)*
            }

            fn distance_to(&self, end: &Self) -> ::std::vec::Vec<::core::primitive::f32> {
                let mut distances = ::std::vec::Vec::with_capacity(Self::components());
                #(#distance_fields)*
                distances.concat()
            }
        }
    };

    TokenStream::from(impl_gen)
}

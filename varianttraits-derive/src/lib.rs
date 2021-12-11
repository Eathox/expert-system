use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

fn varianttraits(ast: &syn::DeriveInput) -> TokenStream {
	let data = &ast.data;

	let variants = if let syn::Data::Enum(syn::DataEnum {
        variants: syn::punctuated::Punctuated { .. },
        ..
    }) = ast.data
    {
        ..
    } else {
        panic!("Only support Enum")
    };

	for variant in variants {

	} 

	let gen = quote! {
		#(
			println!(
				"pub struct {}", #keys
			);
		)*
	};
	eprintln!("{:#?}", ast.data);
	gen.into()
}

#[proc_macro_derive(VariantTraits)]
pub fn varianttraits_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    varianttraits(&ast)
}

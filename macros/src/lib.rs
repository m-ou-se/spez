extern crate proc_macro;

mod parse;

use parse::Args;
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_hack::proc_macro_hack;
use quote::quote;

#[proc_macro_hack]
pub fn spez(tokens: TokenStream) -> TokenStream {
	spez_impl(syn::parse_macro_input!(tokens)).into()
}

fn refs(n: usize) -> TokenStream2 {
	let mut refs = TokenStream2::new();
	for _ in 0..n {
		refs.extend(quote![&]);
	}
	refs
}

fn spez_impl(args: Args) -> TokenStream2 {
	let mut traits = TokenStream2::new();

	let n_arms = args.arms.len();

	for (i, arm) in args.arms.into_iter().enumerate() {
		let name = syn::Ident::new(&format!("Match{}", i + 1), Span::call_site());
		let body = arm.body;
		let ty = arm.ty;
		let generics = &arm.generics;
		let where_clause = &arm.generics.where_clause;
		let refs = refs(n_arms - i - 1);
		traits.extend(quote! {
			trait #name {
				fn spez(&self) #body
			}
			impl #generics #name for #refs Match<#ty> #where_clause {}
		});
	}

	let expr = args.expr;
	let refs = refs(n_arms);

	quote! {
		{
			struct Match<T>(T);
			#traits
			(#refs Match(#expr)).spez()
		}
	}
}

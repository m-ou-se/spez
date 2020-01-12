use syn::Token;

pub struct Args {
	pub for_token: Token![for],
	pub expr: syn::Expr,
	pub semicolon_token: Token![;],
	pub arms: Vec<Arm>,
}

pub struct Arm {
	pub match_token: Token![match],
	pub generics: syn::Generics,
	pub ty: syn::Type,
	pub body: syn::Block,
}

impl syn::parse::Parse for Args {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		Ok(Self {
			for_token: input.parse()?,
			expr: input.parse()?,
			semicolon_token: input.parse()?,
			arms: {
				let mut arms = Vec::new();
				while !input.is_empty() {
					arms.push(input.parse()?);
				}
				arms
			},
		})
	}
}

impl syn::parse::Parse for Arm {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let match_token = input.parse()?;
		let generics = if input.peek(Token![<]) {
			input.parse()?
		} else {
			syn::Generics::default()
		};
		let ty = input.parse()?;
		let where_clause: Option<syn::WhereClause> = input.parse()?;
		let body = input.parse()?;
		Ok(Self {
			match_token,
			generics: syn::Generics {
				where_clause,
				..generics
			},
			ty,
			body,
		})
	}
}

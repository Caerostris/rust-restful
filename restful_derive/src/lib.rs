extern crate proc_macro;
extern crate syn;
extern crate regex;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;
use regex::Regex;

const GET_ATTR_NAME: &'static str = "get";

#[proc_macro_derive(RestResource, attributes(get))]
pub fn restful_resource(input: TokenStream) -> TokenStream {
	let ast = syn::parse_derive_input(&input.to_string()).unwrap();
	let gen = impl_restful_resource(&ast);
	gen.parse().unwrap()
}

fn impl_restful_resource(ast: &syn::DeriveInput) -> quote::Tokens {
	let mut tokens = quote::Tokens::new();
	ast.attrs.iter()
		.filter_map(|attr| match attr.name() {
			GET_ATTR_NAME => Some(impl_get_resource(ast, get_attr_value(attr))),
			_ => None,
		})
		.for_each(|impl_tokens| tokens.append(impl_tokens));
	tokens
}

fn get_attr_value(attr: &syn::Attribute) -> &str {
	if let syn::MetaItem::NameValue(_, syn::Lit::Str(ref value, _)) = attr.value {
		value.as_str()
	} else {
		panic!("#[get] must be of the form #[get = \"...\"]");
	}
}

fn impl_get_resource(ast: &syn::DeriveInput, uri: &str) -> quote::Tokens {
	let name = &ast.ident;
	let parameter_regex = Regex::new(r"\{([^\}]+)\}").unwrap();

	let parameter_types: Vec<&str> = parameter_regex.captures_iter(uri)
		.map(|capture_group| capture_group.get(1).unwrap().as_str())
		.collect();

	let parameter_ident = syn::Ident::from(parameter_types.iter()
		.enumerate()
		.fold(String::new(), |base, (i, p_type)| format!("{}, p{}: {}", base, i, p_type))
	);

	let format_text = parameter_regex.replace_all(uri, "{}");
	let mut format_string = format!("format!(\"{}\"", format_text);

	for i in 0..parameter_types.len() {
		format_string.push_str(format!(", p{}", i).as_str());
	}

	format_string.push_str(").as_str()");

	let uri_format_ident = syn::Ident::from(format_string);

	quote! {
		impl #name {
			fn get(api: &restful::RestApi #parameter_ident) -> Result<#name, restful::Error> {
				api.get_resource(#uri_format_ident)
			}
		}
	}
}

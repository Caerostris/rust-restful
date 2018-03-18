#![recursion_limit="128"]
extern crate proc_macro;
extern crate syn;
extern crate regex;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;
use regex::Regex;

const GET_ATTR_NAME: &'static str = "get";
const POST_ATTR_NAME: &'static str = "post";

#[proc_macro_derive(RestResource, attributes(get, post))]
pub fn restful_resource(input: TokenStream) -> TokenStream {
	let ast = syn::parse_derive_input(&input.to_string()).unwrap();
	let gen = impl_restful_resource(&ast);
	gen.parse().unwrap()
}

fn impl_restful_resource(ast: &syn::DeriveInput) -> quote::Tokens {
	let mut tokens = quote::Tokens::new();
	ast.attrs.iter()
		.filter_map(|attr| match attr.name() {
			GET_ATTR_NAME => Some(impl_get_resource(ast, method_attr_value(GET_ATTR_NAME, attr))),
			POST_ATTR_NAME => Some(impl_post_resource(ast, method_attr_value(POST_ATTR_NAME, attr))),
			_ => None,
		})
		.for_each(|impl_tokens| tokens.append(impl_tokens));
	tokens
}

fn method_attr_value<'a>(method: &'a str, attr: &'a syn::Attribute) -> &'a str {
	if let syn::MetaItem::NameValue(_, syn::Lit::Str(ref value, _)) = attr.value {
		value.as_str()
	} else {
		panic!("#[{}] must be of the form #[{} = \"...\"]", method, method);
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
		#[allow(dead_code)]
		impl #name {
			pub fn get(api: &restful::RestApi #parameter_ident) -> restful::Result<#name, restful::Error> {
				api.get_json(#uri_format_ident)
			}
		}
	}
}

fn impl_post_resource(ast: &syn::DeriveInput, uri: &str) -> quote::Tokens {
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
		#[allow(dead_code)]
		impl #name {
			pub fn post<T>(api: &restful::RestApi #parameter_ident, data: restful::PostData<T>) -> restful::Result<#name, restful::Error>
				where T: restful::Serialize {
				api.post_json(#uri_format_ident, data)
			}

			pub fn post_multipart(api: &restful::RestApi #parameter_ident, data: restful::Form) -> restful::Result<#name, restful::Error> {
				api.post_json_multipart(#uri_format_ident, data)
			}

			pub fn post_empty(api: &restful::RestApi #parameter_ident) -> restful::Result<#name, restful::Error> {
				api.post_json_empty(#uri_format_ident)
			}
		}
	}
}

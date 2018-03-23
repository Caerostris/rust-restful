extern crate reqwest;
extern crate serde;

pub use std::result::Result;
pub use reqwest::{Error, get, Response, IntoUrl};
pub use reqwest::multipart::{Form, Part};
pub use serde::Serialize;

#[derive(Debug, Clone)]
pub struct RestApi {
	base_url: String,
}

pub enum PostData<'a, T: 'a + Serialize> {
	Json(&'a T),
	Form(&'a T),
}

impl RestApi {
	pub fn new(base_url: &str) -> RestApi {
		RestApi {
			base_url: String::from(base_url)
		}
	}

	pub fn get_json<T>(&self, uri: &str) -> Result<T, Error> where for<'de> T: serde::Deserialize<'de> {
		self.get(uri)?.json()
	}

	pub fn get(&self, uri: &str) -> Result<Response, Error> {
		reqwest::get(format!("{}{}", self.base_url, uri).as_str())
	}

	pub fn post_json<T, U>(&self, uri: &str, data: PostData<U>) -> Result<T, Error>
		where for<'de> T: serde::Deserialize<'de>, U: Serialize {
		self.post(uri, data)?.json()
	}

	pub fn post<T: Serialize>(&self, uri: &str, data: PostData<T>) -> Result<Response, Error> {
		let url = format!("{}{}", self.base_url, uri);
		let mut client = reqwest::Client::new().post(url.as_str());

		let client = match data {
			PostData::Json(json) => client.json(json),
			PostData::Form(form) => client.form(form),
		};

		client.send()
	}

	pub fn post_json_multipart<T>(&self, uri: &str, data: Form) -> Result<T, Error> where for<'de> T: serde::Deserialize<'de> {
		self.post_multipart(uri, data)?.json()
	}

	pub fn post_multipart(&self, uri: &str, data: Form) -> Result<Response, Error> {
		reqwest::Client::new()
			.post(format!("{}{}", self.base_url, uri).as_str())
			.multipart(data)
			.send()
	}

	pub fn post_json_empty<T>(&self, uri: &str) -> Result<T, Error> where for<'de> T: serde::Deserialize<'de> {
		self.post_empty(uri)?.json()
	}

	pub fn post_empty(&self, uri: &str) -> Result<Response, Error> {
		reqwest::Client::new()
			.post(format!("{}{}", self.base_url, uri).as_str())
			.send()
	}
}

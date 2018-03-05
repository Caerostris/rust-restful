extern crate reqwest;
extern crate serde;

pub use std::result::Result;
pub use reqwest::{Error, get, Response, IntoUrl};

pub struct RestApi {
	base_url: String,
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
}

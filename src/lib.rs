extern crate reqwest;
extern crate serde;

pub use reqwest::Error;

pub struct RestApi<'a> {
	base_url: &'a str,
}

impl<'a> RestApi<'a> {
	pub fn new(base_url: &'a str) -> RestApi {
		RestApi {
			base_url
		}
	}

	pub fn get_resource<T>(&self, uri: &str) -> Result<T, Error> where for<'de> T: serde::Deserialize<'de> {
		reqwest::get(format!("{}{}", self.base_url, uri).as_str())?.json()
	}
}

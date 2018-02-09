extern crate reqwest;
extern crate serde;

pub use reqwest::Error;

pub struct RestApi {
	base_url: String,
}

impl RestApi {
	pub fn new(base_url: &str) -> RestApi {
		RestApi {
			base_url: String::from(base_url)
		}
	}

	pub fn get_resource<T>(&self, uri: &str) -> Result<T, Error> where for<'de> T: serde::Deserialize<'de> {
		reqwest::get(format!("{}{}", self.base_url, uri).as_str())?.json()
	}
}

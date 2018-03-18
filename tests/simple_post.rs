extern crate restful;
#[macro_use] extern crate restful_derive;
#[macro_use] extern crate serde_derive;

use restful::{RestApi, PostData};
use std::collections::HashMap;

#[derive(Deserialize, RestResource)]
#[post = "/post"]
struct HttpBinPost {
	form: HashMap<String, String>,
	json: Option<HashMap<String, String>>,
}

#[derive(Deserialize, RestResource)]
#[post = "/post?number={i32}"]
struct HttpBinPostQuery {
	args: HashMap<String, String>,
}

#[test]
fn httpbin_test_post_multipart() {
	let api = RestApi::new("http://httpbin.org");
	let multipart = restful::Form::new().text("my_data", "post test");
	let post_data = HttpBinPost::post_multipart(&api, multipart).unwrap();
	assert_eq!(post_data.form["my_data"], "post test");
}

#[test]
fn httpbin_test_post_json() {
	let api = RestApi::new("http://httpbin.org");
	let mut map = HashMap::new();
	map.insert("my_data", "post test");
	let post_data = HttpBinPost::post(&api, PostData::Json(&map)).unwrap();
	assert!(post_data.json.is_some());
	assert_eq!(post_data.json.unwrap()["my_data"], "post test");
}

#[test]
fn httpbin_test_post_form() {
	let api = RestApi::new("http://httpbin.org");
	let mut map = HashMap::new();
	map.insert("my_data", "post test");
	let post_data = HttpBinPost::post(&api, PostData::Form(&map)).unwrap();
	assert_eq!(post_data.form["my_data"], "post test");
}

#[test]
fn httpbin_test_post_query_params() {
	let api = RestApi::new("http://httpbin.org");
	let number = 42;
	let post_data = HttpBinPostQuery::post_empty(&api, number).unwrap();
	assert_eq!(post_data.args["number"], number.to_string());
}
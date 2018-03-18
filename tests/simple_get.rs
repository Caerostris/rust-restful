extern crate restful;
#[macro_use] extern crate restful_derive;
#[macro_use] extern crate serde_derive;

use restful::RestApi;

#[derive(Deserialize, RestResource)]
#[get = "/ip"]
struct HttpBinIp {
	origin: String,
}

#[derive(Deserialize, RestResource)]
#[get = "/get?someVar={u32}"]
struct HttpBinGet {
	url: String,
}

#[test]
fn httpbin_get_ip() {
	let api = RestApi::new("http://httpbin.org");
	let ip = HttpBinIp::get(&api).unwrap();
	assert!(ip.origin.len() > 0);
}

#[test]
fn httpbin_get_params() {
	let api = RestApi::new("http://httpbin.org");
	let some_var = 400;
	let get = HttpBinGet::get(&api, some_var).unwrap();
	assert_eq!(get.url, format!("http://httpbin.org/get?someVar={}", some_var));
}
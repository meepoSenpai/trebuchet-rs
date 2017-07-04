use std::process::{Command, Output};
use std::error::Error;
use ddg::{Query, Response};
use std::option::Option;

const APP_NAME: &'static str = "trebuchet-rs";

pub fn search_for_file(search: &str) -> Result<String, Box<Error>> {
    let output: Output = Command::new("find")
        .arg("/Applications")
        .arg("-name")
        .arg(&search)
        .output()?;
    let as_string: String = String::from_utf8(output.stdout)?;
    Ok(as_string)
}

pub fn instant_reply(search_query: &str) -> Option<Response> {
	let query: Query = Query::new(search_query, APP_NAME);
	match query.execute() {
		Ok(t) => return Some(t),
		Err(_) => return None
	};
}
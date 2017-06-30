use std::process::{Command, Output};
use std::error::Error;

pub fn search_for_file(search: &str) -> Result<String, Box<Error>> {
    let output: Output = Command::new("find")
        .arg("/Applications")
        .arg("-name")
        .arg(&search)
        .output()?;
    let as_string: String = String::from_utf8(output.stdout)?;
    Ok(as_string)
}
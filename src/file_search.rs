use std::process::{Command, Output};
use std::error::Error;

pub fn search_and_print(file_name: &str) {

    println!("Search file! Wait...");
    let search_result_result = search_for_file(file_name);

    match search_result_result {
        Ok(search_result) => {
            println!("Results count: {}", search_result.len());
            for (i, r) in search_result.iter().enumerate() {
                println!("{}: {}", i + 1, &r);
            }
        }
        Err(e) => println!("Error! Details: {:?}", e),
    }
}

//todo return Vec<&str>
pub fn search_for_file<'a>(search: &str) -> Result<Vec<String>, Box<Error>> {
    let output: Output = Command::new("find")
        .arg("/Applications")
        .arg("-name")
        .arg(&search)
        .output()?;
    let as_string: String = String::from_utf8(output.stdout)?;
    let mut result_list = vec![];
    let lines = as_string.lines();
    for line in lines {
        result_list.push(line.to_string());
    }
    Ok(result_list)
}

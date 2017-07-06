use file_search;
use std::error::Error;
use std::process::Command;

pub fn launch_app(path: &str) -> Result<(), Box<Error>> {
    match Command::new(path).output() {
        Ok(_) => println!("Command worked without problem"),
        Err(_) => println!("Failed to launch application."),
    };
    Ok(())
}

pub fn try_launch(name: &str) -> Result<(), Box<Error>> {
    let search_result = file_search::search_for_file(name)?;
    let first = search_result.get(0);
    match first {
        Some(found) => launch_app(found)?,
        None => launch_app("!!!error: app not found ")?,
    }
    Ok(())
}
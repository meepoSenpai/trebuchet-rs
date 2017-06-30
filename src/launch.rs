use search;
use std::error::Error;
use std::process::Command;

fn launch_app(path: &str) -> Result<(), Box<Error>> {
    Command::new("open").arg(path).spawn()?;
    Ok(())
}

pub fn try_launch(name: &str) -> Result<(), Box<Error>> {
    let search_result = search::search_for_file(name)?;

    let first = search_result.lines().next();
    match first {
        Some(found) => launch_app(found)?,
        None => launch_app("!!!error: app not found ")?,
    }
    Ok(())
}
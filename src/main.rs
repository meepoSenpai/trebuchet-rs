extern crate regex;
extern crate ddg;

mod launch;
mod file_search;
mod command_chooser;
mod web_search;

use command_chooser::{Command, match_command};

use std::io::{self, Write};

const VALID_PREFIXES: &'static str = "Valid command-prefixes: ['run:', 'quack:', 'web:', 'file:', 'related:']";


fn main() {
    println!("{}", VALID_PREFIXES);
    print!("Enter the command you desire to run: ");
    io::stdout().flush().unwrap();
    let mut input: String = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => println!("Command read successful\n"),
        _ => println!("Something went wrong"),
    };
    input.pop();
    let result = match_command(&input[..]);

    match result {
        Command::Run(t) => launch::launch_app(t).unwrap(),
        Command::InstantAnswer(t) => web_search::instant_answer_and_print(t),
        Command::WebSearch(t) => println!("Web search! Query: {}", t),
        Command::RelatedTopics(t) => web_search::get_related_and_print(t),
        Command::FileSearch(t) => file_search::search_and_print(t),
        Command::FaultyCommand => println!("Malformed command. {}", VALID_PREFIXES),
    };

}
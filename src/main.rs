extern crate regex;
extern crate ddg;

mod launch;
mod search;
mod command_chooser;

use command_chooser::{Command, match_command};

use std::io::{self, Write};

fn main() {

	print!("Enter the command you desire to run: ");
	io::stdout().flush().unwrap();
    let mut input: String = String::new();
    match io::stdin().read_line(&mut input){
    	Ok(_) => println!("Command read successful\n"),
    	_ => println!("Something went wrong")
    };
    input.pop();
    let result = match_command(&input[..]);

    match result {
    	Command::Run(t) => launch::launch_app(t).unwrap(),
    	Command::InstantAnswer(t) => {
    		let result: ddg::Response = match search::instant_reply(t) {
    			Some(t) => t,
    			None => panic!()
    		};
    		println!("{}", result.heading);
    		println!("{}", result.abstract_text);
    	},
    	Command::WebSearch(_) => println!("DDG something!"),
    	Command::FileSearch(_) => println!("Search file!"),
    	Command::FaultyCommand => println!("Malformed command. Valid command-prefixes: ['run:', 'quack:', 'web:', 'file:']")
    };

}
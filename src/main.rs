extern crate regex;
extern crate ddg;

mod launch;
mod search;
mod command_chooser;

use command_chooser::{Command, match_command};

use std::io;

fn main() {

    let mut input: String = String::new();
    match io::stdin().read_line(&mut input){
    	Ok(_) => println!("Read successful"),
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
    		println!("{}", result.abstract_text);
    	},
    	Command::WebSearch(_) => println!("DDG something!"),
    	Command::FileSearch(_) => println!("Search file!"),
    	Command::FaultyCommand => println!("This was wrong")
    };

}
extern crate regex;
extern crate ddg;

mod launch;
mod search;
mod command_chooser;

use command_chooser::{Command, match_command};
use ddg::RelatedTopic;

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
            match &result.abstract_text[..] {
                "" => {
                    println!("Unclear as to what answer you needed");
                    println!("Possible answers include:");
                    let mut counter = 1;
                    for elem in result.related_topics {
                        
                        match elem {
                            RelatedTopic::TopicResult(t) => {
                                let mut counter = &mut counter;
                                println!("{}. {}", counter, t.text);
                                *counter = *counter + 1;
                            },
                            RelatedTopic::Topic(_) => continue
                        };
                        
                    }
                },
                _ => println!("{}", result.abstract_text)
            };
        },
        Command::WebSearch(_) => println!("DDG something!"),
        Command::FileSearch(_) => println!("Search file!"),
        Command::FaultyCommand => println!("Malformed command. Valid command-prefixes: ['run:', 'quack:', 'web:', 'file:']")
    };

}
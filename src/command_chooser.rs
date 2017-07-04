use regex::Regex;

pub enum Command<'a>{
	Run(&'a str),
	WebSearch(&'a str),
	InstantAnswer(&'a str),
	FileSearch(&'a str),
	FaultyCommand
}


pub fn match_command(command: &str) -> Command {
	let run_command: Regex = Regex::new(r"^run:.*").unwrap();
	let web_command: Regex = Regex::new(r"^web:.*").unwrap();
	let instant_answer: Regex = Regex::new(r"^quack:.*").unwrap();
	let file_search: Regex = Regex::new(r"^file:.*").unwrap();
	
	if run_command.is_match(command){
		return Command::Run(&command[4..]);
	}else if web_command.is_match(command){
		return Command::WebSearch(&command[4..]);
	}else if instant_answer.is_match(command){
		return Command::InstantAnswer(&command[6..]);
	}else if file_search.is_match(command){
		return Command::FileSearch(&command[5..]);
	}
	Command::FaultyCommand
}
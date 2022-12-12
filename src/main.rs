use std::io::{stdout, Write, stdin};
use colored::*;
use cs_lib::{self, generateRoom};

fn main(){
    let stdin = stdin();
    let room = generateRoom();

    loop {
        let mut input = String::from("");

        print!("> ");
        stdout().flush().unwrap();

        stdin.read_line(&mut input).expect("Failed to read line");
        input = (&mut input).trim().to_string();
    
        match input.as_str() {
            "exit" => break,
            "describe"=>println!("{}", room.describe()),
            _ => println!("{}", "invalid command".red()),
        }
    }
}
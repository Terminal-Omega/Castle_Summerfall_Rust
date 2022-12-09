use std::io::{stdout, Write, stdin};
use colored::*;

fn main(){
    let stdin = stdin();

    loop {
        let mut input = String::from("");

        print!("> ");
        stdout().flush().unwrap();

        stdin.read_line(&mut input).expect("Failed to read line");
        input = (&mut input).trim().to_string();

        match input.as_str() {
            "exit" => break,
            _ => println!("{}", "invalid command".red()),
        }
    }
}
#![allow(dead_code)]
use std::io::{self, Write};

const DFA_FILE_PATH: &str = "input/";
const DFA_EXTENSION: &str = ".dfa";

pub fn get_dfa_folder() -> String {
    String::from(DFA_FILE_PATH)
}

pub fn get_dfa_path(file_name: &str) -> String {
    let mut dfa_path = get_dfa_folder();
    dfa_path.push_str(file_name);
    dfa_path.push_str(DFA_EXTENSION);

    dfa_path
}

pub fn read_usize(prompt: &str) -> usize {
    match print_prompt(prompt) {
        Ok(_) => (),
        Err(_) => {
            // we don't care about the error here for now
        }
    };

    let input = match read_line() {
        Ok(input) => input,
        Err(_) => {
            let error = format!("ERROR: Failed to read input!");
            println!("{}", error);
            return read_usize(prompt);
        }
    };

    let number = match input.trim().parse::<usize>() {
        Ok(number) => number,
        Err(_) => {
            let error = format!("ERROR: Invalid input!");
            println!("{}", error);
            return read_usize(prompt);
        }
    };

    number
}

pub fn read_string(prompt: &str) -> String {
    match print_prompt(prompt) {
        Ok(_) => (),
        Err(_) => {
            // we don't care about the error here for now
        }
    };

    let input = match read_line() {
        Ok(input) => input,
        Err(_) => {
            let error = format!("ERROR: Failed to read input!");
            println!("{}", error);
            return read_string(prompt);
        }
    };

    input.trim().to_string()
}

fn print_prompt(prompt: &str) -> Result<(), String> {
    print!("{} ", prompt);
    match io::stdout().flush() {
        Ok(_) => Ok(()),
        Err(e) => {
            let error = format!("flush failed: {}", e.to_string());
            return Err(error);
        }
    }
}

fn read_line() -> Result<String, String> {
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(_) => Ok(line),
        Err(e) => {
            let error = format!("read failed: {}", e.to_string());
            return Err(error);
        }
    }
}

mod models;
mod utils;

use std::env;
use utils::scanner::Scanner;

const TOKEN_FILE_PATH: &str = "../../2/token.in";
const PROGRAM_FILE_PATH: &str = "../../1/";
const PROGRAM_EXTENSION: &str = ".oli";
const DEFAULT_PROGRAM: &str = "p3";

fn get_program_path(program: &str) -> String {
    let mut program_path = String::from(PROGRAM_FILE_PATH);
    program_path.push_str(program);
    program_path.push_str(PROGRAM_EXTENSION);

    program_path
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: cargo run <program name>");
        return;
    }

    let program = if args.len() == 1 {
        get_program_path(DEFAULT_PROGRAM)
    } else {
        get_program_path(&args[1])
    };

    let mut scanner = match Scanner::new(TOKEN_FILE_PATH) {
        Ok(scanner) => scanner,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    match scanner.scan(program.as_str()) {
        Ok(_) => scanner.display(),
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    println!("\nProgram scanned successfully!");
}

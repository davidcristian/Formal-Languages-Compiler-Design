mod models;
mod utils;

use models::scanner::Scanner;
use std::{env, fs};

const PROGRAM_FILE_PATH: &str = "../../programs/";
const PROGRAM_EXTENSION: &str = ".oli";
const DEFAULT_PROGRAM: &str = "p3";

const OUTPUT_DIR: &str = "output/";
const OUTPUT_FILE_PATH: &str = "/tokens";
const OUTPUT_EXTENSION: &str = ".out";

fn get_program_path(program: &str) -> String {
    let mut program_path = String::from(PROGRAM_FILE_PATH);
    program_path.push_str(program);
    program_path.push_str(PROGRAM_EXTENSION);

    program_path
}

fn get_output_path(program: &str) -> Result<String, String> {
    let mut output_path = String::from(OUTPUT_DIR);
    output_path.push_str(program);

    match fs::create_dir_all(&output_path) {
        Ok(_) => {}
        Err(e) => {
            let error = format!("could not create output directory: {}", e);
            return Err(error);
        }
    }

    output_path.push_str(OUTPUT_FILE_PATH);
    output_path.push_str(OUTPUT_EXTENSION);

    Ok(output_path)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: cargo run <program name>");
        return;
    }

    let program = if args.len() == 1 {
        DEFAULT_PROGRAM
    } else {
        &args[1]
    };

    let input_path = get_program_path(program);
    let output_path = match get_output_path(program) {
        Ok(output_path) => output_path,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let mut scanner = match Scanner::new() {
        Ok(scanner) => scanner,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    match scanner.scan(input_path.as_str(), output_path.as_str()) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    println!("\nOutput written to {}", output_path)
}

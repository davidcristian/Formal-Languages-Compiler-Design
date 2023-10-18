use std::fs::File;
use std::io::{self, BufRead};

use crate::models::table::Table;

pub struct Scanner {
    tokens: Vec<String>,

    // store the token table (PIF), identifier table, and constant table separately
    token_table: Table<String>,
    identifier_table: Table<String>,
    constant_table: Table<String>,
}

impl Scanner {
    pub fn new(token_file_path: &str) -> Self {
        let mut scanner = Scanner {
            tokens: vec![],
            token_table: Table::new(),
            identifier_table: Table::new(),
            constant_table: Table::new(),
        };
        scanner.parse_token_file(token_file_path);

        scanner
    }

    fn parse_token_file(&mut self, file_path: &str) {
        println!("Parsing token file '{}'", &file_path);
        let token_file = File::open(file_path).expect("could not open token file");
        let token_reader = io::BufReader::new(token_file);

        let raw_tokens = token_reader
            .lines()
            .map(|line| line.expect("could not read token line"))
            .collect::<Vec<String>>();

        println!("Raw tokens: {:?}", raw_tokens);
        self.tokens = raw_tokens;
    }

    pub fn scan(&self, file: &str) {
        println!("Scanning '{}'", file);
        let program_file = File::open(file).expect("could not open program file");
        let program_reader = io::BufReader::new(program_file);

        // TODO:
        // - use list of tokens that were read from the token file
        // - print lexical errors and their line number

        let mut line_number = 1;
        for line in program_reader.lines() {
            let line = line.expect("could not read program line");
            self.parse_line(&line, line_number);
            line_number += 1;
        }

        println!("\nToken table:");
        self.token_table.display();
        println!("Token table size: {}", self.token_table.size());

        println!("\nIdentifier table:");
        self.identifier_table.display();
        println!("Identifier table size: {}", self.identifier_table.size());

        println!("\nConstant table:");
        self.constant_table.display();
        println!("Constant table size: {}", self.constant_table.size());
    }

    #[allow(unused_variables)]
    fn parse_line(&self, line: &str, line_number: usize) {
        //println!("Parsing line {:>2}: '{}'", line_number, line);
        let mut line_tokens = Vec::new();

        let mut token = String::new();
        let mut is_string = false;

        let mut chars = line.chars().peekable();
        while let Some(c) = chars.next() {
            // handle comment
            if c == '-' && chars.peek() == Some(&'-') && !is_string {
                break;
            }

            // TODO: handle char literals like 'a' and 'b', not only strings
            // handle string start or end
            if c == '"' {
                token.push(c);
                if is_string {
                    line_tokens.push(token.clone());
                    token.clear();
                }

                is_string = !is_string;
                continue;
            }

            // we are in a string, append char regardless of what it is
            if is_string {
                token.push(c);
                continue;
            }

            // not in string mode, handle tokens
            if c == ' ' {
                // handle whitespace
                if !token.is_empty() {
                    line_tokens.push(token.clone());
                    token.clear();
                }
            } else if "=!<>".contains(c) && chars.peek() == Some(&'=') {
                // handle operators like '==', '!=', '<=', '>='
                line_tokens.push(format!("{}{}", c, chars.next().unwrap()));
            } else if "()[]{}:,+-*/%".contains(c) {
                // handle separators and arithmetic operators
                if !token.is_empty() {
                    line_tokens.push(token.clone());
                    token.clear();
                }
                line_tokens.push(c.to_string());
            } else {
                // build up token
                token.push(c);
            }
        }

        // handle last token
        if !token.is_empty() {
            line_tokens.push(token);
        }

        if !line_tokens.is_empty() {
            println!("Tokens: {:?}", line_tokens);
        }

        // TODO: populate tables
    }
}

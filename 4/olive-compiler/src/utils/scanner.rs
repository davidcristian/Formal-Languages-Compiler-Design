use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

use super::token::*;
use crate::models::pair::Pair;
use crate::models::table::Table;

const INTERNAL_SEPARATOR: &str = "\\n";
const IDENTIFIER_NAME: &str = "id";
const CONSTANT_NAME: &str = "constant";

pub struct Scanner {
    reserved_tokens: Vec<String>,
    raw_line: Vec<char>,
    line_index: usize,

    // store the token list, identifier table, and constant table separately
    token_list: Vec<Pair<String, isize>>,
    identifier_table: Table<String>,
    constant_table: Table<String>,
}

impl Scanner {
    pub fn new(token_file_path: &str) -> Self {
        Self {
            reserved_tokens: Self::parse_token_file(token_file_path),
            raw_line: vec![],
            line_index: 0,

            token_list: vec![],
            identifier_table: Table::new(),
            constant_table: Table::new(),
        }
    }

    fn parse_token_file(file_path: &str) -> Vec<String> {
        println!("Reading '{}'", &file_path);
        let token_file = File::open(file_path).expect("could not open token file");
        let token_reader = io::BufReader::new(token_file);

        token_reader
            .lines()
            .map(|line| line.expect("could not read token line"))
            .collect()
    }

    pub fn display(&self) {
        println!("\nToken list:");
        for entry in &self.token_list {
            println!("Token: {}, Table Index: {}", entry.key, entry.value);
        }
        println!("Token list size: {}", self.token_list.len());

        println!("\nIdentifier table:");
        self.identifier_table.display();
        println!("Identifier table size: {}", self.identifier_table.size());

        println!("\nConstant table:");
        self.constant_table.display();
        println!("Constant table size: {}", self.constant_table.size());
    }

    pub fn scan(&mut self, file_path: &str) -> Result<(), String> {
        println!("Scanning '{}'", file_path);
        let program_file = File::open(file_path).expect("could not open program file");
        let program_reader = io::BufReader::new(program_file);

        program_reader
            .lines()
            .enumerate()
            .map(|(line_number, line)| {
                let line = line.expect("could not read program line");
                self.raw_line = line.chars().collect();

                self.line_index = 0;
                self.parse_line(line_number + 1)
            })
            .collect()
    }

    fn parse_line(&mut self, line_number: usize) -> Result<(), String> {
        let mut is_empty = true;

        while let Some(token) = self.next_token() {
            is_empty = false;
            if !self.classify_token(&token) {
                let error = format!("Undefined token on line {}: {}", line_number, token);
                return Err(error);
            }
        }

        if !is_empty {
            self.classify_token(INTERNAL_SEPARATOR);
        }

        Ok(())
    }

    fn next_token(&mut self) -> Option<String> {
        self.consume_whitespace();
        if self.line_index >= self.raw_line.len() {
            return None;
        }

        let current_char = self.raw_line[self.line_index];
        let next_char = self.raw_line.get(self.line_index + 1);

        let token_types: Vec<Box<dyn Token>> = vec![
            Box::new(StringCharToken {}),
            Box::new(CommentToken {}),
            Box::new(ReservedToken::new(&self.reserved_tokens)),
            Box::new(UnclassifiedToken {}),
        ];

        for token_type in token_types {
            if token_type.is_of(current_char, next_char) {
                return token_type.consume(self);
            }
        }

        None
    }

    fn capture_token_stream<F: FnMut(&char) -> bool>(&mut self, mut cond: F) {
        while self.line_index < self.raw_line.len() && cond(&self.raw_line[self.line_index]) {
            self.line_index += 1;
        }
    }

    fn consume_whitespace(&mut self) {
        self.capture_token_stream(|&ch| ch.is_whitespace());
    }

    pub fn consume_string_char(&mut self) -> String {
        let quote_type = self.raw_line[self.line_index];
        let mut token = quote_type.to_string();

        self.line_index += 1;
        self.capture_token_stream(|&ch| {
            token.push(ch);
            ch != quote_type
        });

        token
    }

    pub fn consume_comment(&mut self) {
        self.capture_token_stream(|&ch| ch != '\n');
    }

    pub fn consume_reserved_token(&mut self) -> String {
        let reserved_tokens = self.reserved_tokens.clone();
        let mut token = String::new();

        // handle separators and arithmetic operators while
        // considering the possibility of n-length tokens
        // that begin with the same character (ex: < and <=)
        self.capture_token_stream(|&ch| {
            let potential_token = format!("{}{}", token, ch);
            if reserved_tokens.contains(&potential_token) {
                token.push(ch);
                true
            } else {
                false
            }
        });

        token
    }

    pub fn consume_general_token(&mut self) -> String {
        let reserved_tokens = self.reserved_tokens.clone();
        let mut token = String::new();

        self.capture_token_stream(|&ch| {
            if ch.is_whitespace() || reserved_tokens.contains(&ch.to_string()) {
                false
            } else {
                token.push(ch);
                true
            }
        });

        token
    }

    fn classify_token(&mut self, token: &str) -> bool {
        // Rules:
        // 1. Is it a reserved word or a symbol? : self.tokens
        // 2. Is it an identifier? : uppercase and lowercase letters
        // 3. Is it a constant? : valid number, string, or char
        // 4. None of the above, lexical error

        let identifier = Regex::new(r"^([A-Za-z]+)$").unwrap();
        let number = Regex::new(r"^(((\+|-)?[1-9][0-9]*)|(0))$").unwrap();
        let string_char = Regex::new(r#"^("[^"]*"|'[^']')$"#).unwrap();

        let table_key = token.to_string();
        if self.reserved_tokens.contains(&table_key) || table_key == INTERNAL_SEPARATOR {
            // check if token is a reserved word or a symbol
            self.token_list.push(Pair {
                key: table_key,
                value: -1,
            });

            return true;
        } else if identifier.is_match(token)
            || number.is_match(token)
            || string_char.is_match(token)
        {
            let (table, key_name) = if identifier.is_match(token) {
                (&mut self.identifier_table, IDENTIFIER_NAME)
            } else {
                (&mut self.constant_table, CONSTANT_NAME)
            };

            // only add to table if element doesn't exist
            let mut value = *table.get(&table_key).unwrap_or(&-1);
            if value == -1 {
                value = table.insert(table_key);
            }

            self.token_list.push(Pair {
                key: String::from(key_name),
                value,
            });

            return true;
        }

        false
    }
}

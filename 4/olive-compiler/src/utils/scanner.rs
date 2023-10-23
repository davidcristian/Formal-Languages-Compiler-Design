use lazy_static::lazy_static;
use regex::Regex;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

use super::token::*;
use crate::models::pair::Pair;
use crate::models::table::Table;

// TODO:
// - fix triple reserved_tokens clone (use refs)
// - make consume functions private and pass
//   references to them instead of the scanner
// - add comments

const INTERNAL_SEPARATOR: &str = "\\n";
const IDENTIFIER_NAME: &str = "id";
const CONSTANT_NAME: &str = "constant";

lazy_static! {
    // these expressions are correct, unwrap() is safe; if not, the compiler will panic
    static ref IDENTIFIER: Regex = Regex::new(r"^([A-Za-z]+)$").unwrap();
    static ref NUMBER: Regex = Regex::new(r"^(((\+|-)?[1-9][0-9]*)|(0))$").unwrap();
    static ref STRING_CHAR: Regex = Regex::new(r#"^("[^"]*"|'[^']')$"#).unwrap();
}

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
    pub fn new(token_file_path: &str) -> Result<Self, String> {
        match Self::parse_token_file(token_file_path) {
            Ok(tokens) => Ok(Self {
                reserved_tokens: tokens,
                raw_line: vec![],
                line_index: 0,

                token_list: vec![],
                identifier_table: Table::new(),
                constant_table: Table::new(),
            }),
            Err(e) => Err(e),
        }
    }

    fn parse_token_file(file_path: &str) -> Result<Vec<String>, String> {
        println!("Reading '{}'", &file_path);
        let token_file = match File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                let error = format!("could not open token file: {}", e.to_string());
                return Err(error);
            }
        };

        let token_reader = BufReader::new(token_file);
        let mut tokens = Vec::new();

        for (line_index, line) in token_reader.lines().enumerate() {
            match line {
                Ok(line) => tokens.push(line),
                Err(e) => {
                    let error = format!(
                        "could not read token file line {}: {}",
                        line_index + 1,
                        e.to_string()
                    );
                    return Err(error);
                }
            }
        }

        Ok(tokens)
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
        let program_file = match File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                let error = format!("could not open program file: {}", e.to_string());
                return Err(error);
            }
        };

        let program_reader = io::BufReader::new(program_file);
        program_reader
            .lines()
            .enumerate()
            .map(|(line_index, line)| {
                self.raw_line = match line {
                    Ok(line) => line.chars().collect(),
                    Err(e) => {
                        let error = format!(
                            "could not read program file line {}: {}",
                            line_index + 1,
                            e.to_string()
                        );
                        return Err(error);
                    }
                };

                self.line_index = 0;
                self.parse_line(&line_index)
            })
            .collect()
    }

    fn parse_line(&mut self, line_index: &usize) -> Result<(), String> {
        let mut is_empty = true;

        while let Some(token) = self.next_token() {
            is_empty = false;
            match self.classify_token(&token) {
                Ok(_) => {}
                Err(e) => {
                    let error = format!("Lexical error on line {} => {}", line_index + 1, e);
                    return Err(error);
                }
            }
        }

        if !is_empty {
            if let Err(_) = self.classify_token(INTERNAL_SEPARATOR) {
                let error = format!("Invalid internal separator");
                return Err(error);
            }
        }

        Ok(())
    }

    fn next_token(&mut self) -> Option<String> {
        self.consume_whitespace();
        if self.line_index >= self.raw_line.len() {
            return None;
        }

        let current_char = &self.raw_line[self.line_index];
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
        let mut token = String::from(quote_type);

        self.line_index += 1;
        self.capture_token_stream(|&ch| {
            token.push(ch);
            ch != quote_type
        });

        self.line_index += 1;
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

    fn classify_token(&mut self, token: &str) -> Result<(), String> {
        // Rules:
        // 1. Is it a reserved word or a symbol? : self.tokens
        // 2. Is it an identifier? : uppercase and lowercase letters
        // 3. Is it a constant? : valid number, string, or char
        // 4. None of the above, lexical error

        let table_key = String::from(token);
        match token {
            _ if self.reserved_tokens.contains(&table_key) || table_key == INTERNAL_SEPARATOR => {
                // check if token is a reserved word or a symbol
                self.token_list.push(Pair {
                    key: table_key,
                    value: -1,
                });

                Ok(())
            }
            _ if IDENTIFIER.is_match(token)
                || NUMBER.is_match(token)
                || STRING_CHAR.is_match(token) =>
            {
                let (table, key_name) = if IDENTIFIER.is_match(token) {
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

                Ok(())
            }
            _ => {
                let error = format!("undefined token: {}", token);
                Err(error)
            }
        }
    }
}

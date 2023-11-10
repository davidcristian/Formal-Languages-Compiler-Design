use lazy_static::lazy_static;
use regex::Regex;

use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};

use super::token::*;
use automata::Automaton;
use hash_map::HashMap;
use hash_map::Table;

// TODO:
// - remove reserved_tokens clone in token.rs
// - make consume functions private and pass references to them instead of the scanner
// - change the method for consuming reserved tokens to work with n-length tokens that do not have a common prefix
// - add comments
// - actually just rewrite this whole thing without reading from token.in

const RESERVED_TOKEN_VALUE: &usize = &0;
const IDENTIFIER_OFFSET: &usize = &1;
const CONSTANT_OFFSET: &usize = &2;

const INTERNAL_SEPARATOR_OFFSET: &usize = &3;
// internal separators are not added if a newline is encountered
// and the last token is part of the INTERNAL_SEP_IGNORED list
const INTERNAL_SEP_IGNORED: &[&str] = &["{"];

lazy_static! {
    // these expressions are correct, unwrap() is safe; if not, the program will panic
    // static ref IDENTIFIER: Regex = Regex::new(r"^([A-Za-z]+)$").unwrap();
    // static ref NUMBER: Regex = Regex::new(r"^(((\+|-)?[1-9][0-9]*)|(0))$").unwrap();
    static ref STRING_CHAR: Regex = Regex::new(r#"^("[^"]*"|'[^']')$"#).unwrap();
}

pub struct Scanner {
    reserved_tokens: HashMap<String, usize>,
    ignored_sep_list: Vec<usize>,

    // store the token list, identifier table, and constant table separately
    token_list: Vec<(usize, usize)>,
    identifier_table: Table<String>,
    constant_table: Table<String>,

    raw_program: Vec<char>,
    position: usize,
    line_index: usize,

    identifier: Automaton,
    number: Automaton,
}

impl Scanner {
    pub fn new(token_file_path: &str) -> Result<Self, String> {
        let identifier_automaton =
            match Automaton::new("../../5/olive-compiler/input/identifier.dfa") {
                Ok(automaton) => automaton,
                Err(e) => {
                    let error = format!("[identifier] {}", e);
                    return Err(error);
                }
            };

        let number_automaton = match Automaton::new("../../5/olive-compiler/input/number.dfa") {
            Ok(automaton) => automaton,
            Err(e) => {
                let error = format!("[number] {}", e);
                return Err(error);
            }
        };

        match Self::parse_token_file(token_file_path) {
            Ok(tokens) => {
                let mut scanner = Self {
                    reserved_tokens: tokens,
                    ignored_sep_list: vec![],

                    token_list: vec![],
                    identifier_table: Table::new(),
                    constant_table: Table::new(),

                    raw_program: vec![],
                    position: 0,
                    line_index: 0,

                    identifier: identifier_automaton,
                    number: number_automaton,
                };

                // iterate ignored tokens for the internal separator and add value from reserved_tokens
                for &token in INTERNAL_SEP_IGNORED {
                    if let Some(value) = scanner.reserved_tokens.get(&token.to_string()) {
                        scanner.ignored_sep_list.push(*value);
                    } else {
                        let error = format!("undefined token in const array: {}", token);
                        return Err(error);
                    }
                }

                Ok(scanner)
            }
            Err(e) => Err(e),
        }
    }

    fn parse_token_file(file_path: &str) -> Result<HashMap<String, usize>, String> {
        println!("Reading '{}'", &file_path);
        let token_file = match File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                let error = format!("could not open token file: {}", e.to_string());
                return Err(error);
            }
        };

        let token_reader = BufReader::new(token_file);
        let mut tokens = HashMap::new();

        for (line_index, line) in token_reader.lines().enumerate() {
            match line {
                Ok(line) => tokens.insert(line, line_index + 1),
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

    fn write_output(&self, file_path: &str, status: &str) -> Result<(), String> {
        let mut file = match File::create(file_path) {
            Ok(file) => file,
            Err(e) => {
                let error = format!("could not create output file: {}", e.to_string());
                return Err(error);
            }
        };

        let mut output = String::new();
        let token_list = self
            .token_list
            .iter()
            .map(|entry| format!("({:2}, {:2})", entry.0, entry.1))
            .collect::<Vec<String>>()
            .join("\n");

        output.push_str("Token list:\n");
        output.push_str(&token_list);
        output.push_str("\nToken list size: ");
        output.push_str(&self.token_list.len().to_string());

        output.push_str("\n\nIdentifier table:\n");
        output.push_str(&self.identifier_table.to_string());
        output.push_str("\nIdentifier table size: ");
        output.push_str(&self.identifier_table.size().to_string());

        output.push_str("\n\nConstant table:\n");
        output.push_str(&self.constant_table.to_string());
        output.push_str("\nConstant table size: ");
        output.push_str(&self.constant_table.size().to_string());

        output.push_str("\n\n");
        output.push_str(status);
        output.push_str("\n");

        match file.write_all(output.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => {
                let error = format!("could not write to output file: {}", e.to_string());
                Err(error)
            }
        }
    }

    pub fn scan(&mut self, input_file: &str, output_file: &str) -> Result<(), String> {
        println!("Scanning '{}'", input_file);
        self.raw_program = match fs::read_to_string(input_file) {
            Ok(program) => program.chars().collect(),
            Err(e) => {
                let error = format!("could not read program file: {}", e.to_string());
                return Err(error);
            }
        };

        self.position = 0;
        self.line_index = 1;

        match self.parse_program() {
            Ok(_) => self.write_output(output_file, "Lexically correct!"),
            Err(e) => self.write_output(output_file, e.as_str()),
        }
    }

    fn parse_program(&mut self) -> Result<(), String> {
        while let Some(token) = self.next_token() {
            match self.classify_token(&token) {
                Ok(_) => {}
                Err(e) => {
                    let error = format!("Lexical error on line {} => {}", self.line_index, e);
                    return Err(error);
                }
            }
        }

        Ok(())
    }

    fn next_token(&mut self) -> Option<String> {
        self.consume_whitespace();
        if self.position >= self.raw_program.len() {
            return None;
        }

        let current_char = &self.raw_program[self.position];
        let next_char = self.raw_program.get(self.position + 1);

        let token_types: Vec<Box<dyn Token>> = vec![
            Box::new(StringCharToken {}),
            Box::new(CommentToken {}),
            Box::new(ReservedToken::new(&self.reserved_tokens)),
            Box::new(UnclassifiedToken {}),
        ];

        for token_type in token_types {
            if token_type.is_of(current_char, next_char) {
                return Some(token_type.consume(self));
            }
        }

        None
    }

    fn capture_token_stream<F: FnMut(&char) -> bool>(&self, mut cond: F) -> usize {
        let mut position = self.position;
        while position < self.raw_program.len() && cond(&self.raw_program[position]) {
            position += 1;
        }

        position
    }

    fn consume_whitespace(&mut self) {
        // count newlines and add them to the line index
        let mut newlines = 0;
        self.position = self.capture_token_stream(|&ch| {
            if ch == '\n' {
                newlines += 1;
            }

            ch.is_whitespace()
        });

        // add a separator token if there was at least one newline
        if newlines > 0 {
            self.add_separator_token();
        }

        self.line_index += newlines;
    }

    pub fn consume_string_char(&mut self) -> String {
        let quote_type = self.raw_program[self.position];
        let mut token = String::from(quote_type);

        self.position += 1;
        self.position = self.capture_token_stream(|&ch| {
            token.push(ch);
            ch != quote_type
        });

        self.position += 1;
        token
    }

    pub fn consume_comment(&mut self) -> String {
        self.position = self.capture_token_stream(|&ch| ch != '\n');
        String::from("")
    }

    pub fn consume_reserved_token(&mut self) -> String {
        let mut token = String::new();

        // handle separators and arithmetic operators,
        // including numbers that begin with + or -, while
        // considering the possibility of n-length tokens
        // that have a common suffix (ex: <, <=, or <==)
        self.position = self.capture_token_stream(|&ch| {
            let potential_token = format!("{}{}", token, ch);
            let first_char = match potential_token.chars().next() {
                Some(ch) => ch,
                None => '\0',
            };

            if self.reserved_tokens.contains(&potential_token)
                || ("+-".contains(first_char) && ch.is_ascii_digit())
            {
                token.push(ch);
                true
            } else {
                false
            }
        });

        token
    }

    pub fn consume_general_token(&mut self) -> String {
        let mut token = String::new();

        self.position = self.capture_token_stream(|&ch| {
            if ch.is_whitespace() || self.reserved_tokens.contains(&ch.to_string()) {
                false
            } else {
                token.push(ch);
                true
            }
        });

        token
    }

    fn add_separator_token(&mut self) {
        let last_token_code = match self.token_list.last() {
            Some(pair) => pair.0,
            None => return,
        };

        // check if the last token is part of the INTERNAL_SEP_IGNORED list
        if self.ignored_sep_list.contains(&last_token_code) {
            return;
        }

        self.token_list.push((
            self.reserved_tokens.size() + INTERNAL_SEPARATOR_OFFSET,
            *RESERVED_TOKEN_VALUE,
        ));
    }

    fn classify_token(&mut self, token: &str) -> Result<(), String> {
        if token.is_empty() {
            // likely a comment
            return Ok(());
        }

        // Rules:
        // 1. Is it a reserved word or a symbol? : self.tokens
        // 2. Is it an identifier? : uppercase and lowercase letters
        // 3. Is it a constant? : valid number, string, or char
        // 4. None of the above, lexical error

        let table_key = String::from(token);
        if let Some(token_code) = self.reserved_tokens.get(&table_key) {
            // check if token is a reserved word or a symbol
            self.token_list.push((*token_code, *RESERVED_TOKEN_VALUE));

            return Ok(());
        } else if self.identifier.validate(token)
            || self.number.validate(token)
            || STRING_CHAR.is_match(token)
        {
            // check if token is an identifier or a constant
            let (table, offset) = if self.identifier.validate(token) {
                (&mut self.identifier_table, IDENTIFIER_OFFSET)
            } else {
                (&mut self.constant_table, CONSTANT_OFFSET)
            };

            // only add to table if element doesn't exist
            let value = match table.get(&table_key) {
                Some(value) => *value,
                None => table.put(table_key),
            };

            let token_code = self.reserved_tokens.size() + offset;
            self.token_list.push((token_code, value));

            return Ok(());
        }

        let error = format!("undefined token: {}", token);
        Err(error)
    }
}

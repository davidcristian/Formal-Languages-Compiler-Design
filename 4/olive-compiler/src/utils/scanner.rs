use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

use crate::models::pair::Pair;
use crate::models::table::Table;

const INTERNAL_SEPARATOR: &str = "\\n";
const IDENTIFIER_NAME: &str = "id";
const CONSTANT_NAME: &str = "constant";

pub struct Scanner {
    tokens: Vec<String>,

    // store the token list, identifier table, and constant table separately
    token_list: Vec<Pair<String, isize>>,
    identifier_table: Table<String>,
    constant_table: Table<String>,
}

impl Scanner {
    pub fn new(token_file_path: &str) -> Self {
        Self {
            tokens: Self::parse_token_file(token_file_path),
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
                self.parse_line(&line, line_number + 1)
            })
            .collect()
    }

    fn parse_line(&mut self, line: &str, line_number: usize) -> Result<(), String> {
        let tokens = self.tokenize_line(line);

        for token in &tokens {
            if !self.validate_token(token) {
                let error = format!("Undefined token on line {}: {}", line_number, token);
                return Err(error);
            }
        }

        Ok(())
    }

    fn tokenize_line(&mut self, line: &str) -> Vec<String> {
        let mut line_tokens = Vec::new();

        let mut token = String::new();
        let (mut is_string, mut is_char) = (false, false);

        // inline function to capture token
        let mut handle_token = |token: &mut String| {
            if !token.is_empty() {
                line_tokens.push(token.clone());
                token.clear();
            }
        };

        let mut chars = line.chars().peekable();
        while let Some(ch) = chars.next() {
            // handle string and char start or end
            if (ch == '"' && !is_char) || (ch == '\'' && !is_string) {
                token.push(ch);

                let is_end_quote = if ch == '"' { is_string } else { is_char };
                if is_end_quote {
                    handle_token(&mut token);
                }

                if ch == '"' {
                    is_string = !is_string;
                } else {
                    is_char = !is_char;
                }

                continue;
            }

            // we are in a string or char, append char regardless of what it is
            if is_string || is_char {
                token.push(ch);
                continue;
            }

            // not in string mode or char mode, handle tokens
            if ch == '-' && chars.peek() == Some(&'-') {
                // handle comment
                break;
            }

            if ch == ' ' {
                // handle whitespace
                handle_token(&mut token);
            } else if self.tokens.contains(&ch.to_string()) {
                // handle separators and arithmetic operators while
                // considering the possibility of n-length tokens
                // that begin with the same character (ex: < and <=)
                handle_token(&mut token);

                token.push(ch);
                while let Some(next_ch) = chars.peek() {
                    let potential_token = token.clone() + &next_ch.to_string();
                    if self.tokens.contains(&potential_token) {
                        token.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                handle_token(&mut token);
            } else {
                // build up token
                token.push(ch);
            }
        }

        // handle last token
        handle_token(&mut token);

        if !line_tokens.is_empty() {
            line_tokens.push(String::from(INTERNAL_SEPARATOR));

            // temporary print statement
            println!("Tokens: {:?}", line_tokens);
        }

        line_tokens
    }

    fn validate_token(&mut self, token: &str) -> bool {
        // Rules:
        // 1. Is it a reserved word or a symbol? : self.tokens
        // 2. Is it an identifier? : uppercase and lowercase letters
        // 3. Is it a constant? : valid number, string, or char
        // 4. None of the above, lexical error

        let rule_2 = Regex::new(r"^([A-Za-z]+)$").unwrap();
        let rule_3_number = Regex::new(r"^(((\+|-)?[1-9][0-9]*)|(0))$").unwrap();
        let rule_3_string_char = Regex::new(r#"^("[^"]*"|'[^']')$"#).unwrap();

        // TODO: verify if char is only one character and if identifier exists in table?

        let table_key = token.to_string();
        if self.tokens.contains(&table_key) || table_key == INTERNAL_SEPARATOR {
            // check if token is a reserved word or a symbol
            self.token_list.push(Pair {
                key: table_key,
                value: -1,
            });

            true
        } else if rule_2.is_match(token) {
            // only add identifier to table if it doesn't exist
            if self.identifier_table.get(&table_key).is_none() {
                self.identifier_table.insert(table_key.clone());
            }

            self.token_list.push(Pair {
                key: String::from(IDENTIFIER_NAME),
                value: self.identifier_table.get(&table_key).unwrap().clone(),
            });

            true
        } else if rule_3_number.is_match(token) || rule_3_string_char.is_match(token) {
            // only add constant to table if it doesn't exist
            if self.constant_table.get(&table_key).is_none() {
                self.constant_table.insert(table_key.clone());
            }

            self.token_list.push(Pair {
                key: String::from(CONSTANT_NAME),
                value: self.constant_table.get(&table_key).unwrap().clone(),
            });

            true
        } else {
            false
        }
    }
}

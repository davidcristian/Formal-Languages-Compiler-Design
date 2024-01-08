use std::fs;

use super::automata::Automata;
use hash_map::Table;

use super::token::{Token, TokenKind};
use crate::utils::constants::{EOF_CHAR, LINE_COMMENT, NEWLINE_CHAR};
use crate::utils::writer::write_scan_result;

pub struct Scanner {
    raw_program: Vec<char>,
    final_result: String,
    position: usize,
    current_line: usize,

    // store the token list, identifier table, and constant table separately
    token_list: Vec<Token>,
    identifier_table: Table<String>,
    constant_table: Table<String>,
    automata: Automata,
}

impl Scanner {
    pub fn new() -> Result<Self, String> {
        let automata = match Automata::new() {
            Ok(automata) => automata,
            Err(e) => {
                return Err(e);
            }
        };

        let scanner = Self {
            raw_program: vec![],
            final_result: String::new(),
            position: 0,
            current_line: 1,

            token_list: vec![],
            identifier_table: Table::new(),
            constant_table: Table::new(),
            automata,
        };

        Ok(scanner)
    }

    pub fn get_token_list(&self) -> &Vec<Token> {
        &self.token_list
    }

    pub fn get_identifier_table(&self) -> &Table<String> {
        &self.identifier_table
    }

    pub fn get_constant_table(&self) -> &Table<String> {
        &self.constant_table
    }

    fn get_nth(&self, n: usize) -> &char {
        match self.raw_program.get(self.position + n) {
            Some(character) => character,
            None => EOF_CHAR,
        }
    }

    fn current(&self) -> &char {
        self.get_nth(0)
    }

    fn peek(&self) -> &char {
        self.get_nth(1)
    }

    fn advance(&mut self, n: usize) {
        self.position += n;
    }

    pub fn scan(&mut self, input_file: &str) -> Result<(), String> {
        println!("Scanning '{}'", input_file);
        self.raw_program = match fs::read_to_string(input_file) {
            // Ok(program) => program.replace("\r\n", "\n").chars().collect(),
            Ok(program) => program.chars().collect(),
            Err(e) => {
                let error = format!("could not read program file: {}", e.to_string());
                return Err(error);
            }
        };

        // reset the scanner
        self.position = 0;
        self.current_line = 1;
        self.final_result.clear();

        self.token_list.clear();
        self.identifier_table.clear();
        self.constant_table.clear();

        // parse the program
        self.final_result = match self.parse_program() {
            Ok(_) => String::from("Lexically correct!"),
            Err(e) => e,
        };

        Ok(())
    }

    pub fn write_result(&self, output_file: &str) -> Result<(), String> {
        write_scan_result(
            output_file,
            &self.final_result,
            &self.token_list,
            &self.identifier_table,
            &self.constant_table,
        )
    }

    fn parse_program(&mut self) -> Result<(), String> {
        // keep reading tokens until we reach EOF
        while let Some(token) = self.next_token() {
            match token.get_kind() {
                // unknown tokens produce lexical errors
                TokenKind::Unknown => {
                    let error = format!(
                        "Lexical error on line {} => undefined token: {}",
                        self.current_line,
                        token.get_inner()
                    );
                    return Err(error);
                }
                // valid token
                _ => self.token_list.push(token),
            }
        }

        Ok(())
    }

    fn next_token(&mut self) -> Option<Token> {
        self.consume_whitespace();
        self.consume_comments();

        // check if we reached EOF
        let current = *self.current();
        if current == *EOF_CHAR {
            return None;
        }

        // we are relying on Unknown tokens to be consumed
        // so that we can catch lexical errors after classifying
        // them. Those do not pass the DFA remain Unknown tokens

        // start parsing the token
        self.advance(1);
        let next = self.current();

        let mut token_value = String::from(current);
        let kind = match current {
            // Special Symbols and Operators
            '+' => match next.is_ascii_digit() {
                true => TokenKind::Unknown,
                false => TokenKind::Plus,
            },
            '-' => match next.is_ascii_digit() {
                true => TokenKind::Unknown,
                false => TokenKind::Minus,
            },
            '*' => TokenKind::Multiply,
            '/' => TokenKind::Divide,
            '%' => TokenKind::Modulo,
            '=' => match next {
                &'=' => {
                    token_value.push(*next);
                    self.advance(1);

                    TokenKind::Equal
                }
                _ => TokenKind::Assign,
            },
            '!' => match next {
                &'=' => {
                    token_value.push(*next);
                    self.advance(1);

                    TokenKind::NotEqual
                }
                _ => TokenKind::Unknown,
            },
            '<' => match next {
                &'=' => {
                    token_value.push(*next);
                    self.advance(1);

                    TokenKind::LessEqual
                }
                _ => TokenKind::Less,
            },
            '>' => match next {
                &'=' => {
                    token_value.push(*next);
                    self.advance(1);

                    TokenKind::GreaterEqual
                }
                _ => TokenKind::Greater,
            },
            '&' => match next {
                &'&' => {
                    token_value.push(*next);
                    self.advance(1);

                    TokenKind::And
                }
                _ => TokenKind::Unknown,
            },
            '|' => match next {
                &'|' => {
                    token_value.push(*next);
                    self.advance(1);

                    TokenKind::Or
                }
                _ => TokenKind::Unknown,
            },

            // Separators
            '(' => TokenKind::ParenOpen,
            ')' => TokenKind::ParenClose,
            '{' => TokenKind::BraceOpen,
            '}' => TokenKind::BraceClose,
            '[' => TokenKind::BracketOpen,
            ']' => TokenKind::BracketClose,
            ',' => TokenKind::Comma,
            ';' => TokenKind::SemiColon,
            ':' => TokenKind::Colon,

            // Other
            '\'' => TokenKind::Constant,
            '"' => TokenKind::Constant,
            _ => TokenKind::Unknown,
        };

        // if the token is unknown, consume the rest of the characters
        let token = match kind {
            TokenKind::Unknown => self.consume_general(&current),
            TokenKind::Constant => self.consume_literal(&current),
            _ => Token::new(kind, &token_value),
        };

        Some(token)
    }

    fn consume_general(&mut self, start: &char) -> Token {
        let mut value = String::from(*start);
        while self.current().is_alphanumeric() {
            value.push(*self.current());
            self.advance(1);
        }

        // check if the token is a keyword
        let mut token = Token::unknown(&value);
        token.classify(&self.automata);

        match token.get_kind() {
            TokenKind::Identifier => {
                let value = self.identifier_table.put(value);
                token.set_position(value);
            }
            TokenKind::Constant => {
                let value = self.constant_table.put(value);
                token.set_position(value);
            }
            _ => {}
        }

        token
    }

    fn consume_literal(&mut self, quote_type: &char) -> Token {
        let mut value = String::from(*quote_type);
        while self.current() != quote_type {
            value.push(*self.current());
            self.advance(1);
        }

        // add the closing quote
        value.push(*self.current());
        self.advance(1);

        // check if the literal is valid
        let mut token = Token::unknown(&value);
        token.classify(&self.automata);

        match token.get_kind() {
            TokenKind::Constant => {
                let value = self.constant_table.put(value);
                token.set_position(value);
            }
            _ => {}
        }

        token
    }

    fn consume_comments(&mut self) {
        // check if we have a valid comment
        let token = format!("{}{}", self.current(), self.peek());
        if token != LINE_COMMENT {
            return;
        }

        // keep reading until we reach a newline or EOF
        while self.current() != NEWLINE_CHAR && self.current() != EOF_CHAR {
            self.advance(1);
        }

        // consume the whitespace characters after the comment
        self.consume_whitespace();
    }

    fn consume_whitespace(&mut self) {
        // keep reading until we reach a non-whitespace character
        while self.current().is_whitespace() {
            if self.current() == NEWLINE_CHAR {
                self.current_line += 1;
            }

            self.advance(1);
        }
    }
}

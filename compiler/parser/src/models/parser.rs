use hash_map::HashMap;
use lazy_static::lazy_static;
use std::collections::HashSet as Set;

use std::fs::File;
use std::io::{BufReader, Lines};
use utils::{extract_line_data, get_next_line, open_file, InputLine};

lazy_static! {
    static ref ESCAPES: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert(r"ε", "");
        map.insert(r"\s", " ");
        map.insert(r"\p", "|");
        map
    };
}

pub struct Parser {
    non_terminals: Set<String>,
    terminals: Set<String>,
    start_symbol: String,
    productions: HashMap<String, Vec<String>>,
}

impl Parser {
    pub fn new(file_path: &str) -> Result<Self, String> {
        let mut parser = Self {
            non_terminals: Set::new(),
            terminals: Set::new(),
            start_symbol: String::new(),
            productions: HashMap::new(),
        };

        match parser.parse_file(file_path) {
            Ok(_) => Ok(parser),
            Err(e) => Err(e),
        }
    }

    pub fn get_non_terminals(&self) -> &Set<String> {
        &self.non_terminals
    }

    pub fn get_terminals(&self) -> &Set<String> {
        &self.terminals
    }

    pub fn get_start_symbol(&self) -> &String {
        &self.start_symbol
    }

    pub fn get_productions(&self) -> &HashMap<String, Vec<String>> {
        &self.productions
    }

    #[allow(dead_code)]
    pub fn get_production(&self, non_terminal: &str) -> Option<&Vec<String>> {
        self.productions.get(&String::from(non_terminal))
    }

    pub fn is_context_free(&self) -> bool {
        for (non_terminal, productions) in &self.productions {
            // ensure the left-hand side is a single non-terminal
            // example: "S" from "S -> a A | a C"
            if !self.non_terminals.contains(non_terminal)
                || non_terminal.split_whitespace().count() > 1
            {
                return false;
            }

            // iterate over each derivation
            // example: ["a A", "a C"] from "S -> a A | a C"
            for production in productions {
                // get the symbols from the derivation
                // example: ["a", "A"] from "a A", and ["a", "C"] from "a C"
                let symbols: Vec<String> =
                    production.split_whitespace().map(String::from).collect();

                // check if each symbol is a terminal or a non-terminal
                for symbol in &symbols {
                    if !self.terminals.contains(symbol) && !self.non_terminals.contains(symbol) {
                        // symbol is neither a terminal nor a non-terminal
                        return false;
                    }
                }
            }
        }

        // grammar is context free
        true
    }

    fn parse_file(&mut self, file_path: &str) -> Result<(), String> {
        let mut lines = match open_file(file_path) {
            Ok(lines) => lines,
            Err(e) => return Err(e),
        };

        // parse non-terminals
        match self.parse_non_terminals(lines.next()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        // parse terminals
        match self.parse_terminals(lines.next()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        // parse start symbol
        match self.parse_start_symbol(lines.next()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        // parse productions
        match self.parse_productions(&mut lines) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        Ok(())
    }

    fn parse_non_terminals(&mut self, non_terminals: InputLine) -> Result<(), String> {
        // check if set of non-terminals is missing
        let non_terminals = match extract_line_data(non_terminals) {
            Some(non_terminals) => non_terminals,
            None => {
                let error = format!("invalid grammar file: missing set of non-terminals");
                return Err(error);
            }
        };

        // add each non-terminal to the set of non-terminals
        for non_terminal in non_terminals.split_whitespace() {
            // insert non-terminal if it doesn't already exist
            match self.non_terminals.insert(String::from(non_terminal)) {
                true => (),
                false => {
                    let error = format!(
                        "duplicate non-terminal '{}' in set of non-terminals",
                        non_terminal
                    );
                    return Err(error);
                }
            }
        }

        Ok(())
    }

    fn parse_terminals(&mut self, terminals: InputLine) -> Result<(), String> {
        // check if set of terminals is missing
        let terminals = match extract_line_data(terminals) {
            Some(terminals) => terminals,
            None => {
                let error = format!("invalid grammar file: missing set of terminals");
                return Err(error);
            }
        };

        // add each terminal to the set of terminals
        for terminal in terminals.split_whitespace() {
            let value = if let Some(escape) = ESCAPES.get(&terminal) {
                escape
            } else {
                terminal
            };

            // insert terminal if it doesn't already exist
            match self.terminals.insert(String::from(value)) {
                true => (),
                false => {
                    let error = format!("duplicate terminal '{}' in set of terminals", terminal);
                    return Err(error);
                }
            }
        }

        Ok(())
    }

    fn parse_start_symbol(&mut self, start_symbol: InputLine) -> Result<(), String> {
        // check if start symbol is missing
        let start_symbol = match extract_line_data(start_symbol) {
            Some(start_symbol) => start_symbol,
            None => {
                let error = format!("invalid grammar file: missing start symbol");
                return Err(error);
            }
        };

        // check if start symbol is a non-terminal
        if !self.non_terminals.contains(&start_symbol) {
            let error = format!(
                "start symbol '{}' not in set of non-terminals",
                start_symbol
            );
            return Err(error);
        }

        // parse start symbol
        self.start_symbol = String::from(start_symbol);
        Ok(())
    }

    fn parse_productions(
        &mut self,
        productions: &mut Lines<BufReader<File>>,
    ) -> Result<(), String> {
        loop {
            // get the next line from the reader
            let line = match get_next_line(productions) {
                Ok(line) => line,
                Err(e) => return Err(e),
            };

            // check if the end of the file has been reached
            let line = match line {
                Some(line) => line,
                None => break,
            };

            // skip empty lines
            if line.is_empty() {
                continue;
            }

            // split the line into a non-terminal and a production
            let parts: Vec<&str> = line.split("->").collect();
            if parts.len() != 2 {
                let error = format!("invalid production: '{}'", line);
                return Err(error);
            }

            // obtain the left-hand side and right-hand side
            let non_terminal = parts[0].trim();
            let productions = parts[1].trim();

            // check if non-terminal is empty
            if non_terminal.is_empty() {
                let error = format!("empty non-terminal in production: '{}'", line);
                return Err(error);
            }

            // check if the non-terminal already has a production
            if self.productions.contains_key(&String::from(non_terminal)) {
                let error = format!("duplicate production key for production: '{}'", line);
                return Err(error);
            }

            // check if the production is empty
            if productions.is_empty() {
                let error = format!("empty production for line: '{}'", line);
                return Err(error);
            }

            // split the productions string into derivations
            let productions: Vec<String> = productions
                .split("|")
                .map(|s| {
                    let derivation = s.trim();
                    if let Some(&escape) = ESCAPES.get(&derivation) {
                        String::from(escape)
                    } else {
                        String::from(derivation)
                    }
                })
                .collect();

            // insert production
            self.productions
                .insert(String::from(non_terminal), productions);
        }

        // check if there are no productions
        if self.productions.len() == 0 {
            let error = format!("invalid grammar file: missing productions");
            return Err(error);
        }

        Ok(())
    }
}

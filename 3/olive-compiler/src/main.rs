mod models;
use models::symbol_table::SymbolTable;

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn parse_line(
    line: &str,
    symbol_table: &mut SymbolTable<String>,
    constant_table: &mut SymbolTable<String>,
    separators: &str,
) {
    // TODO: fix for last part of for loops
    // first part matches simple assignments like a = 2
    // second part matches declarations like a: string = "abc", b: number
    let variable_regex_pattern = format!(
        r#"([0-9A-Za-z]+\s*=\s*("[^"]*"|'[^']*'|[^{}]*)$)|([0-9A-Za-z]+\s*:\s*(number|char|string)(\s*=\s*("[^"]*"|'[^']*'|[^{}]*))?)"#,
        separators, separators
    );

    // example captures: ["n: number = 1", "c: char = 'a'", "s: string"]
    let regex = Regex::new(variable_regex_pattern.as_str()).unwrap();
    for capture in regex.captures_iter(&line) {
        println!("\ncapture: {:?}", &capture[0]);
        if capture[0].trim().ends_with("=") {
            panic!("invalid declaration: {}", &capture[0])
        }

        // TODO: check if it is a comparison (==, >, <, >=, <=)
        // if yes, ignore

        let mut split_capture = capture[0].split("=");
        // extract left hand side (declaration) and right hand side (value)
        let extracted_declaration = split_capture.next().unwrap().trim();
        let symbol_value = split_capture.next().unwrap_or("").trim();

        let mut split_declaration = extracted_declaration.split(":");
        // extract left hand side (symbol name) and right hand side (symbol type)
        let symbol_name = split_declaration.next().unwrap().trim();
        let symbol_type = split_declaration.next().unwrap_or("").trim();

        if !is_identifier(&symbol_name) {
            panic!("invalid symbol name: {}", symbol_name);
        }

        // this is a simple declaration, example: a = 2
        if symbol_type.is_empty() {
            if symbol_table.get(&symbol_name.to_string()).is_none() {
                panic!("identifier not in symbol table: {}", symbol_name);
            }

            // parse the value of the assignment
            parse_value(&symbol_value, symbol_table, constant_table);
            return;
        }

        // perhaps remove this in the future when user-defined types are added
        if !symbol_type.chars().all(|c| c.is_lowercase()) {
            panic!("invalid symbol type: {}", symbol_type);
        }

        println!("declaration: {}", extracted_declaration);
        println!("symbol_type: {}", symbol_type);
        println!("symbol_name: {}", symbol_name);
        println!("symbol_value: {}", symbol_value);

        // parse the value of the declaration
        parse_value(&symbol_value, symbol_table, constant_table);

        // add symbol to symbol table if it doesn't exist
        if symbol_table.get(&symbol_name.to_string()).is_none() {
            symbol_table.put(symbol_name.to_string());
        }
    }
}

fn parse_value(
    value: &str,
    symbol_table: &mut SymbolTable<String>,
    constant_table: &mut SymbolTable<String>,
) {
    // check if value is missing (empty string) => simple declaration like "a: number"
    // an empty string token would still be a valid value and passes this check
    // this check is added to cover truly-empty strings that came from unwrap_or("")
    if value.is_empty() {
        return;
    }

    // this pattern intentionally matches a lot of number formats that will be filtered out later
    const VALUE_REGEX_PATTERN: &str =
        r#"(".*?("|$))|('.*?('|$))|(((\+|-)*)?(\d*\.\d+|\d+\.?\d*))|([A-Za-z]+)"#;

    // example captures: ["a + 2", "5 + 6 + 7", "b - c", "'a' + 'b'"]
    let regex = Regex::new(VALUE_REGEX_PATTERN).unwrap();
    for capture in regex.captures_iter(&value) {
        let constant = &capture[0];

        if is_identifier(constant) {
            println!("- found identifier: {}", constant);

            // check if identifier exists in the symbol table
            if symbol_table.get(&constant.to_string()).is_none() {
                panic!("identifier not in symbol table: {}", constant);
            }
        } else {
            println!("- found constant: {}", constant);

            if validate_value(constant) {
                // add constant to constant table if it doesn't exist
                if constant_table.get(&constant.to_string()).is_none() {
                    constant_table.put(constant.to_string());
                }
            } else {
                // this should match all invalid constants such as floats, invalid numbers, etc.
                panic!("- invalid constant: {}", constant);
            }
        }
    }
}

fn validate_value(value: &str) -> bool {
    // rules:
    // 1. if value starts with ', it must end with ' and only contain one character inside (ascii space through tilde)
    // 2. if value starts with ", it must end with " and may be empty or contain one or more characters inside (ascii space through tilde)
    // 3. else, the value must be a valid number (cannot start with zero unless it is zero, may start with either + or -)

    // rule 1
    let rule_1 = Regex::new(r#"^'[ -~]'$"#).unwrap();

    // rule 2
    let rule_2 = Regex::new(r#"^"[ -~]*"$"#).unwrap();

    // rule 3
    let rule_3 = Regex::new(r#"^([+-]?[1-9][0-9]*|0)$"#).unwrap();

    rule_1.is_match(value) || rule_2.is_match(value) || rule_3.is_match(value)
}

fn is_identifier(value: &str) -> bool {
    // rule:
    // - value must only contain letters (case-insensitive)

    let rule = Regex::new(r#"^[A-Za-z]+$"#).unwrap();
    rule.is_match(value)
}

fn escape_character_classes(tokens: &Vec<char>) -> String {
    // escapes needed characters in a regex character class
    const ESCAPE_CHARACTERS: &str = r#"^-]\["#;
    let mut escaped_value = String::new();

    for &ch in tokens {
        if ESCAPE_CHARACTERS.contains(ch) {
            escaped_value.push('\\');
        }

        escaped_value.push(ch);
    }

    escaped_value
}

fn main() {
    const PROGRAM_FILE_PATH: &str = "../../1/p2.oli";
    const TOKEN_FILE_PATH: &str = "../../2/token.in";

    let token_file = File::open(TOKEN_FILE_PATH).expect("could not open token file");
    let token_reader = io::BufReader::new(token_file);

    // read tokens from token file
    let raw_tokens = token_reader
        .lines()
        .map(|line| line.expect("could not read token line"))
        .collect::<Vec<String>>();

    // keep only unique special characters (not alphanumeric)
    // example: from ["<", "<=", "char", ","] to ["<", "=", ","]
    let tokens = raw_tokens
        .iter()
        .filter(|&token| !token.chars().all(|ch| ch.is_alphanumeric()))
        .flat_map(|token| token.chars())
        .collect::<std::collections::HashSet<char>>()
        .into_iter()
        .collect::<Vec<char>>();

    println!("tokens: {:?}", &tokens);

    // define program separators
    let separators: Vec<char> = vec!['<', '>', '(', ')', '[', ']', '{', '}', ':', ';', ','];
    let separators = escape_character_classes(&separators);

    // store the symbol table and constant table separately
    let mut symbol_table: SymbolTable<String> = SymbolTable::new();
    let mut constant_table: SymbolTable<String> = SymbolTable::new();

    let program_file = File::open(PROGRAM_FILE_PATH).expect("could not open program file");
    let program_reader = io::BufReader::new(program_file);

    // read program line by line
    for line in program_reader.lines() {
        let line = line.expect("could not read program line");
        parse_line(&line, &mut symbol_table, &mut constant_table, &separators);
    }

    println!("\nSymbol Table");
    symbol_table.display();
    println!("Symbol Table Size: {}", symbol_table.size());

    println!("\nConstant Table");
    constant_table.display();
    println!("Constant Table Size: {}", constant_table.size());
}

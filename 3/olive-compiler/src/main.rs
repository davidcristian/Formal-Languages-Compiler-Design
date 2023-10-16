mod models;
use models::symbol_table::SymbolTable;

use regex::Regex;
use std::fs;

fn parse_program(
    program: &str,
    symbol_table: &mut SymbolTable<String>,
    constant_table: &mut SymbolTable<String>,
) {
    // TODO: 2 more cases:
    // - simple assigment AFTER declaration: a = 2
    // - that last case with .+, only have it .+ for non-strings, otherwise it must be in quotes

    // this pattern is case-insensitive
    const INIT_REGEX_PATTERN: &str =
        r#"(?i)(const\s+)?[0-9A-Za-z]+\s*:\s*(void|number|char|string)(\s*=\s*.+)?"#;

    // example captures: ["n: number = 1", "const s: string = "abc"", "c: char"]
    let regex = Regex::new(INIT_REGEX_PATTERN).unwrap();
    for capture in regex.captures_iter(&program) {
        println!("\ncapture: {:?}", &capture[0]);

        let mut split_capture = capture[0].split("=");
        // extract left hand side (declaration) and right hand side (value)
        let extracted_declaration = split_capture.next().unwrap().trim();
        let symbol_value = split_capture.next().unwrap_or("").trim();

        let mut split_declaration = extracted_declaration.split(":");
        // extract left hand side (symbol name) and right hand side (symbol type)
        let extracted_name = split_declaration.next().unwrap().trim();
        let symbol_type = split_declaration.next().unwrap().trim();

        // parse the symbol name and check if it is constant
        let (symbol_name, is_constant) = parse_symbol_name(extracted_name);

        println!("declaration: {}", extracted_declaration);
        println!("symbol_type: {}", symbol_type);
        println!("symbol_name: {}", symbol_name);
        println!("symbol_value: {}", symbol_value);
        println!("is_mutable: {}", !is_constant);

        // add symbol to symbol table if it doesn't exist
        if symbol_table.get(&symbol_name.to_string()).is_none() {
            symbol_table.put(symbol_name.to_string());
        }

        // parse the value of the declaration
        parse_value(&symbol_value, constant_table);
    }
}

fn parse_symbol_name(extracted_name: &str) -> (String, bool) {
    // check if the declaration is for a constant variable (preceded by "const")
    // by default we assume that the variable is not constant and the extracted name is correct
    let mut symbol_name = extracted_name.to_string();
    let mut split_name = extracted_name.split_whitespace();

    let first_token = split_name.next().unwrap();
    let is_constant = first_token.to_lowercase() == "const";
    if is_constant {
        // "const" was not all lowercase => throw error
        if first_token != "const" {
            panic!("invalid token preceding symbol name: {}", extracted_name);
        }

        // symbol name is the next token after const
        symbol_name = split_name.next().unwrap().to_string();

        // if split_name still has elements, then the declaration is invalid => throw error
        // this is to prevent a declaration like "const a b c: number = 1"
        if split_name.next().is_some() {
            panic!("invalid symbol name: {}", extracted_name);
        }
    }

    (symbol_name, is_constant)
}

fn parse_value(value: &str, constant_table: &mut SymbolTable<String>) {
    // check if value is not empty
    // an empty string token would still be a valid value and passes this check
    // this check is added to cover truly-empty strings that came from unwrap_or("")
    if value.is_empty() {
        return;
    }

    // this pattern intentionally matches a lot of number formats that will be filtered out later
    const VALUE_REGEX_PATTERN: &str =
        r#"(".*?("|$))|('.*?('|$))|(((\+|-)*)?(\d*\.\d+|\d+\.?\d*))|([A-Za-z]+)"#;

    // example captures: ["a + 2", "5 + 6 + 7", "b - c", ""abc" + "def""]
    let regex = Regex::new(VALUE_REGEX_PATTERN).unwrap();
    for capture in regex.captures_iter(&value) {
        let constant = &capture[0];
        println!("- found constant: {}", constant);

        // TODO: also match identifiers and check if they exist in the symbol table?

        if validate_value(constant) {
            // add constant to constant table if it doesn't exist
            if constant_table.get(&constant.to_string()).is_none() {
                constant_table.put(constant.to_string());
            }
        } else {
            // identifiers are intentionally not matched for the constant table
            println!("- invalid constant: {}", constant);
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

fn main() {
    const FILE_PATH: &str = "../../1/p2.oli";
    let program = fs::read_to_string(FILE_PATH).expect("ERROR: Could not read file!");

    // store the symbol table and constant table separately
    let mut symbol_table: SymbolTable<String> = SymbolTable::new();
    let mut constant_table: SymbolTable<String> = SymbolTable::new();

    parse_program(&program, &mut symbol_table, &mut constant_table);

    println!("\nSymbol Table");
    symbol_table.display();

    println!("\nConstant Table");
    constant_table.display();
}

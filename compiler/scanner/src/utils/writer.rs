use hash_map::Table;
use std::{fs::File, io::Write};

use crate::models::token::Token;

pub fn write_scan_result(
    file_path: &str,
    result: &String,
    token_list: &Vec<Token>,
    identifier_table: &Table<String>,
    constant_table: &Table<String>,
) -> Result<(), String> {
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(e) => {
            let error = format!("could not create output file: {}", e.to_string());
            return Err(error);
        }
    };

    let mut output = String::new();
    let tokens = token_list
        .iter()
        .map(|entry| {
            // println!(
            //     "{:2}, {:2} -> {}",
            //     entry.key(),
            //     entry.value(),
            //     entry.get_inner().replace("\n", "\\n").replace("\0", "\\0")
            // );

            format!("({:2}, {:2})", entry.key(), entry.value())
        })
        .collect::<Vec<String>>()
        .join("\n");

    output.push_str("Token list:\n");
    output.push_str(&tokens);
    output.push_str("\nToken list size: ");
    output.push_str(&token_list.len().to_string());

    output.push_str("\n\nIdentifier table:\n");
    for (key, value) in identifier_table {
        output.push_str(&format!("K: {:?}, V: {}\n", key, value));
    }
    output.push_str("Identifier table size: ");
    output.push_str(&identifier_table.len().to_string());

    output.push_str("\n\nConstant table:\n");
    for (key, value) in constant_table {
        output.push_str(&format!("K: {:?}, V: {}\n", key, value));
    }
    output.push_str("Constant table size: ");
    output.push_str(&constant_table.len().to_string());

    output.push_str("\n\n");
    output.push_str(result);
    output.push_str("\n");

    match file.write_all(output.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => {
            let error = format!("could not write to output file: {}", e.to_string());
            Err(error)
        }
    }
}

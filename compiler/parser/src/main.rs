mod models;
mod tests;

use models::grammar::Grammar;
use models::parser::LL1Parser;
use scanner::Scanner;

#[allow(dead_code)]
fn print_first_follow(grammar: &Grammar) {
    // sort non-terminals by key
    let mut non_terminals: Vec<&String> = grammar.get_non_terminals().iter().collect();
    non_terminals.sort();

    // sort terminals by key but keep shorter terminals first
    let mut terminals: Vec<&String> = grammar.get_terminals().iter().collect();
    terminals.sort_by(|a, b| {
        if a.len() == b.len() {
            return a.cmp(b);
        }
        if a.len() < b.len() {
            return std::cmp::Ordering::Less;
        }
        std::cmp::Ordering::Greater
    });

    // calculate first and follow for each terminal
    for terminal in terminals.iter() {
        let first = grammar.first(terminal);
        let follow = grammar.follow(terminal);
        println!(
            "first('{}') = {}",
            terminal,
            first
                .iter()
                .map(|s| format!("'{}'", s))
                .collect::<Vec<String>>()
                .join(", ")
        );
        println!(
            "follow('{}') = {}",
            terminal,
            follow
                .iter()
                .map(|s| format!("'{}'", s))
                .collect::<Vec<String>>()
                .join(", ")
        );
    }

    // calculate first and follow for each non-terminal
    for non_terminal in non_terminals.iter() {
        let first = grammar.first(non_terminal);
        let follow = grammar.follow(non_terminal);
        println!(
            "first('{}') = {}",
            non_terminal,
            first
                .iter()
                .map(|s| format!("'{}'", s))
                .collect::<Vec<String>>()
                .join(", ")
        );
        println!(
            "follow('{}') = {}",
            non_terminal,
            follow
                .iter()
                .map(|s| format!("'{}'", s))
                .collect::<Vec<String>>()
                .join(", ")
        );
    }
}

fn main() {
    let grammar = match Grammar::new("input/grammar.in") {
        Ok(grammar) => grammar,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // sort non-terminals by key
    let mut non_terminals: Vec<&String> = grammar.get_non_terminals().iter().collect();
    non_terminals.sort();

    print!("{} Non-Terminals: ", non_terminals.len());
    println!(
        "{}",
        non_terminals
            .iter()
            .map(|s| format!("'{}'", s))
            .collect::<Vec<String>>()
            .join(", ")
    );

    // sort terminals by key but keep shorter terminals first
    let mut terminals: Vec<&String> = grammar.get_terminals().iter().collect();
    terminals.sort_by(|a, b| {
        if a.len() == b.len() {
            return a.cmp(b);
        }
        if a.len() < b.len() {
            return std::cmp::Ordering::Less;
        }
        std::cmp::Ordering::Greater
    });

    print!("\n{} Terminals: ", terminals.len());
    println!(
        "{}",
        terminals
            .iter()
            .map(|s| format!("'{}'", s))
            .collect::<Vec<String>>()
            .join(", ")
    );

    println!("\nStart Symbol: '{}'", grammar.get_start_symbol());

    // sort productions by key
    let mut productions: Vec<(&String, &Vec<String>)> = grammar.get_productions().iter().collect();
    productions.sort_by(|a, b| a.0.cmp(b.0));

    print!("\n{} Productions:\n", productions.len());
    for (non_terminal, production) in productions {
        println!(
            "  '{}': {}",
            non_terminal,
            production
                .iter()
                .map(|s| format!("'{}'", s))
                .collect::<Vec<String>>()
                .join(", ")
        );
    }

    println!("\nFirst and Follow Sets:");
    print_first_follow(&grammar);

    println!("\ngrammar is context free: {}\n", grammar.is_context_free());
    let parser = LL1Parser::new(grammar);

    let mut table: Vec<(&(String, String), &String)> = parser.get_parsing_table().iter().collect();
    table.sort_by(|a, b| {
        if a.0 .0 == b.0 .0 {
            return a.0 .1.cmp(&b.0 .1);
        }
        a.0 .0.cmp(&b.0 .0)
    });

    println!("Parsing Table:");
    for ((non_terminal, terminal), production) in &table {
        println!("({}, {}) -> '{}'", non_terminal, terminal, production);
    }
    println!("Parsing Table Size: {}", table.len());

    println!("\nFinished building parsing table.");

    let mut scanner = match Scanner::new() {
        Ok(scanner) => scanner,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let program = "../../programs/p3.oli";
    match scanner.scan(program) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    println!("Finished scanning input.\n");
    let tokens = scanner.get_token_list();

    let identifiers = scanner.get_identifier_table();
    let constants = scanner.get_constant_table();

    let output = match parser.parse(&tokens, &identifiers, &constants) {
        Ok(output) => output,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    output.display();
    match output.write_output("output/parse_tree.out") {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    println!("\nFinished parsing input.");
}

mod models;
mod tests;

use models::parser::Parser;

#[allow(dead_code)]
fn print_first_follow(parser: &Parser) {
    // sort non-terminals by key
    let mut non_terminals: Vec<&String> = parser.get_non_terminals().iter().collect();
    non_terminals.sort();

    // sort terminals by key but keep shorter terminals first
    let mut terminals: Vec<&String> = parser.get_terminals().iter().collect();
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
        let first = parser.first(terminal);
        let follow = parser.follow(terminal);
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
        let first = parser.first(non_terminal);
        let follow = parser.follow(non_terminal);
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
    println!("The LL(1) Parser is not implemented yet.");
    let parser = match Parser::new("input/grammar.in") {
        Ok(parser) => parser,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // sort non-terminals by key
    let mut non_terminals: Vec<&String> = parser.get_non_terminals().iter().collect();
    non_terminals.sort();

    print!("\n{} Non-Terminals: ", non_terminals.len());
    println!(
        "{}",
        non_terminals
            .iter()
            .map(|s| format!("'{}'", s))
            .collect::<Vec<String>>()
            .join(", ")
    );

    // sort terminals by key but keep shorter terminals first
    let mut terminals: Vec<&String> = parser.get_terminals().iter().collect();
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

    println!("\nStart Symbol: '{}'", parser.get_start_symbol());

    // sort productions by key
    let mut productions: Vec<(&String, &Vec<String>)> = parser.get_productions().iter().collect();
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

    // print_first_follow(&parser);
    println!("\ngrammar is context free: {}\n", parser.is_context_free());

    let tests = ["ll_pass", "pass_1", "pass_2", "fail_1", "fail_2"];
    for file in tests.iter() {
        let parser = match Parser::new(&format!("input/{}.in", file)) {
            Ok(parser) => parser,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
        println!("{} is context free: {}", file, parser.is_context_free());
    }
}

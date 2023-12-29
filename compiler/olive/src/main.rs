use parser::{Grammar, LL1Parser};
use scanner::Scanner;

fn main() {
    let grammar_file = "../parser/input/ll_pass.in";
    let grammar = match Grammar::new(grammar_file) {
        Ok(grammar) => grammar,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let parser = LL1Parser::new(grammar);

    let mut scanner = match Scanner::new() {
        Ok(scanner) => scanner,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let program = "../parser/input/program.test";
    match scanner.scan(program) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    println!("Finished scanning input.");
    println!("\nParsing input...");

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

    match output.write_output("output/parse_tree.out") {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    println!("Finished parsing input.");
}

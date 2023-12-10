use parser::{Grammar, LL1Parser};
use scanner::Scanner;

fn main() {
    println!("The compiler is not fully implemented yet.");
    return;

    let grammar_file = "../parser/input/grammar.in";
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

    let program = "../../programs/p3.oli";
    match scanner.scan(program) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    println!("Finished scanning input.");
    let tokens = scanner.get_token_list();

    let output = match parser.parse(&tokens) {
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

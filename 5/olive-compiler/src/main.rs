mod utils;
use utils::automaton::Automaton;

fn main() {
    let idetifiers = vec!["hello", "hell2o", ""];
    let numbers = vec![
        "123", "0123", "+1213", "+-1213", "-+1213", "-123", "-0", "+0", "+023", "+014", "",
    ];

    let identifier = match Automaton::new("dfa/identifier.in") {
        Ok(identifier) => identifier,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let number = match Automaton::new("dfa/number.in") {
        Ok(number) => number,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    for value in idetifiers {
        if identifier.validate(value) {
            println!("Value '{}' is a VALID identifier", value);
        } else {
            println!("Value '{}' is NOT an identifier", value);
        }
    }

    println!();

    for value in numbers {
        if number.validate(value) {
            println!("Value '{}' is a VALID number", value);
        } else {
            println!("Value '{}' is NOT a number", value);
        }
    }

    println!("\nDone!");
}

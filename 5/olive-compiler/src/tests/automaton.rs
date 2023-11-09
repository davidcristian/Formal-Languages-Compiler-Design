#[allow(unused_imports)]
use crate::utils::automaton::Automaton;

#[test]
fn test_identifier() {
    let good_idetifiers = vec!["num", "s"];
    let bad_idetifiers = vec!["num2", "@s", ""];

    let identifier = Automaton::new("dfa/identifier.in").unwrap();

    for value in good_idetifiers {
        assert!(identifier.validate(value));
    }

    for value in bad_idetifiers {
        assert!(!identifier.validate(value));
    }
}

#[test]
fn test_number() {
    let good_numbers = vec!["1", "10", "+12", "-12", "0"];
    let bad_numbers = vec!["01", "+-10", "-+10", "+0", "-0", "+01", "-01", ""];

    let number = Automaton::new("dfa/number.in").unwrap();

    for value in good_numbers {
        assert!(number.validate(value));
    }

    for value in bad_numbers {
        assert!(!number.validate(value));
    }
}

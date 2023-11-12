#[allow(unused_imports)]
use crate::utils::automaton::Automaton;

#[test]
fn test_identifier() {
    let good_idetifiers = vec!["num", "s"];
    let bad_idetifiers = vec!["num2", "@s", ""];

    let identifier = Automaton::new("input/identifier.dfa").unwrap();

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

    let number = Automaton::new("input/number.dfa").unwrap();

    for value in good_numbers {
        assert!(number.validate(value));
    }

    for value in bad_numbers {
        assert!(!number.validate(value));
    }
}

#[test]
fn test_string() {
    let good_strings = vec![
        r#""abc""#,
        r#""a""#,
        r#""a \"b\" c""#,
        r#""a \\ b""#,
        r#""""#,
    ];
    let bad_strings = vec![
        r#""a " b""#,
        r#""a "bc" d""#,
        r#""a \ b""#,
        r#""abc"#,
        r#"abc""#,
    ];

    let string = Automaton::new("input/string.dfa").unwrap();

    for value in good_strings {
        assert!(string.validate(value));
    }

    for value in bad_strings {
        assert!(!string.validate(value));
    }
}

#[test]
fn test_char() {
    let good_chars = vec!["'a'", "' '", r#"'\''"#, r#"'\\'"#];
    let bad_chars = vec!["'ab'", "''", r#"'''"#, r#"'\'"#];

    let char = Automaton::new("input/char.dfa").unwrap();

    for value in good_chars {
        assert!(char.validate(value));
    }

    for value in bad_chars {
        assert!(!char.validate(value));
    }
}

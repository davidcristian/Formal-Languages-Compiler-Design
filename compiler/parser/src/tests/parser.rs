#[allow(unused_imports)]
use std::collections::HashSet as Set;

#[allow(unused_imports)]
use crate::models::parser::Parser;

#[test]
fn test_pass_1() {
    let parser = Parser::new("input/pass_1.in").unwrap();

    let non_terminals = parser.get_non_terminals();
    let terminals = parser.get_terminals();
    let productions = parser.get_productions();

    assert_eq!(non_terminals.len(), 5);
    assert_eq!(terminals.len(), 3);
    assert_eq!(productions.len(), 5);

    let start_symbol = parser.get_start_symbol();
    assert_eq!(start_symbol, "S");

    assert_eq!(parser.is_context_free(), true);
}

#[test]
fn test_pass_2() {
    let parser = Parser::new("input/pass_2.in").unwrap();

    let non_terminals = parser.get_non_terminals();
    let terminals = parser.get_terminals();
    let productions = parser.get_productions();

    assert_eq!(non_terminals.len(), 2);
    assert_eq!(terminals.len(), 2);
    assert_eq!(productions.len(), 2);

    let start_symbol = parser.get_start_symbol();
    assert_eq!(start_symbol, "A");

    assert_eq!(parser.is_context_free(), true);
}

#[test]
fn test_fail_1() {
    let parser = Parser::new("input/fail_1.in").unwrap();

    let non_terminals = parser.get_non_terminals();
    let terminals = parser.get_terminals();
    let productions = parser.get_productions();

    assert_eq!(non_terminals.len(), 3);
    assert_eq!(terminals.len(), 2);
    assert_eq!(productions.len(), 2);

    let start_symbol = parser.get_start_symbol();
    assert_eq!(start_symbol, "S");

    assert_eq!(parser.is_context_free(), false);
}

#[test]
fn test_fail_2() {
    let parser = Parser::new("input/fail_2.in").unwrap();

    let non_terminals = parser.get_non_terminals();
    let terminals = parser.get_terminals();
    let productions = parser.get_productions();

    assert_eq!(non_terminals.len(), 2);
    assert_eq!(terminals.len(), 2);
    assert_eq!(productions.len(), 2);

    let start_symbol = parser.get_start_symbol();
    assert_eq!(start_symbol, "S");

    assert_eq!(parser.is_context_free(), false);
}

#[test]
fn test_first_follow_1() {
    let parser = Parser::new("input/ll_pass.in").unwrap();

    assert_eq!(parser.first("S"), Set::from([String::from("a")]));
    assert_eq!(parser.first("A"), Set::from([String::from("b")]));
    assert_eq!(
        parser.first("A'"),
        Set::from([String::from("b"), String::from("ε")])
    );
    assert_eq!(parser.first("B"), Set::from([String::from("b")]));
    assert_eq!(
        parser.first("C"),
        Set::from([String::from("a"), String::from("b")])
    );
    assert_eq!(
        parser.first("C'"),
        Set::from([String::from("b"), String::from("ε")])
    );
    assert_eq!(parser.first("D"), Set::from([String::from("b")]));

    assert_eq!(parser.follow("S"), Set::from([String::from("$")]));
    assert_eq!(parser.follow("A"), Set::from([String::from("$")]));
    assert_eq!(parser.follow("A'"), Set::from([String::from("$")]));
    assert_eq!(
        parser.follow("B"),
        Set::from([String::from("b"), String::from("$")])
    );
    assert_eq!(parser.follow("C"), Set::from([String::from("$")]));
    assert_eq!(parser.follow("C'"), Set::from([String::from("$")]));
    assert_eq!(
        parser.follow("D"),
        Set::from([String::from("b"), String::from("$")])
    );
}

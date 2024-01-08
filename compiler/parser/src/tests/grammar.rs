#[allow(unused_imports)]
use std::collections::HashSet as Set;

#[allow(unused_imports)]
use crate::models::grammar::Grammar;

#[test]
fn test_cfg() {
    let pass = ["ll_pass", "pass_1", "pass_2"];
    let fail = ["fail_1", "fail_2"];

    for file in pass.iter() {
        let grammar = Grammar::new(&format!("input/{}.in", file)).unwrap();
        assert!(grammar.is_context_free());
    }

    for file in fail.iter() {
        let grammar = Grammar::new(&format!("input/{}.in", file)).unwrap();
        assert!(!grammar.is_context_free());
    }
}

#[test]
fn test_pass_1() {
    let grammar = Grammar::new("input/pass_1.in").unwrap();

    let non_terminals = grammar.get_non_terminals();
    let terminals = grammar.get_terminals();
    let productions = grammar.get_productions();

    assert_eq!(non_terminals.len(), 5);
    assert_eq!(terminals.len(), 3);
    assert_eq!(productions.len(), 5);

    let start_symbol = grammar.get_start_symbol();
    assert_eq!(start_symbol, "S");

    assert_eq!(grammar.is_context_free(), true);
}

#[test]
fn test_pass_2() {
    let grammar = Grammar::new("input/pass_2.in").unwrap();

    let non_terminals = grammar.get_non_terminals();
    let terminals = grammar.get_terminals();
    let productions = grammar.get_productions();

    assert_eq!(non_terminals.len(), 2);
    assert_eq!(terminals.len(), 2);
    assert_eq!(productions.len(), 2);

    let start_symbol = grammar.get_start_symbol();
    assert_eq!(start_symbol, "A");

    assert_eq!(grammar.is_context_free(), true);
}

#[test]
fn test_fail_1() {
    let grammar = Grammar::new("input/fail_1.in").unwrap();

    let non_terminals = grammar.get_non_terminals();
    let terminals = grammar.get_terminals();
    let productions = grammar.get_productions();

    assert_eq!(non_terminals.len(), 3);
    assert_eq!(terminals.len(), 2);
    assert_eq!(productions.len(), 2);

    let start_symbol = grammar.get_start_symbol();
    assert_eq!(start_symbol, "S");

    assert_eq!(grammar.is_context_free(), false);
}

#[test]
fn test_fail_2() {
    let grammar = Grammar::new("input/fail_2.in").unwrap();

    let non_terminals = grammar.get_non_terminals();
    let terminals = grammar.get_terminals();
    let productions = grammar.get_productions();

    assert_eq!(non_terminals.len(), 2);
    assert_eq!(terminals.len(), 2);
    assert_eq!(productions.len(), 2);

    let start_symbol = grammar.get_start_symbol();
    assert_eq!(start_symbol, "S");

    assert_eq!(grammar.is_context_free(), false);
}

#[test]
fn test_first_follow_1() {
    let grammar = Grammar::new("input/ll_pass.in").unwrap();

    assert_eq!(
        grammar.first("S"),
        Set::from([String::from("("), String::from(")"), String::from("ε")])
    );
    assert_eq!(
        grammar.first("A"),
        Set::from([String::from("("), String::from(")"), String::from("ε")])
    );
    assert_eq!(
        grammar.first("B"),
        Set::from([String::from("("), String::from(")")])
    );
    assert_eq!(grammar.first("("), Set::from([String::from("(")]));
    assert_eq!(grammar.first(")"), Set::from([String::from(")")]));

    assert_eq!(grammar.follow("S"), Set::from([String::from("$")]));
    assert_eq!(grammar.follow("A"), Set::from([String::from("$")]));
    assert_eq!(
        grammar.follow("B"),
        Set::from([String::from("("), String::from(")"), String::from("$")])
    );
    assert_eq!(
        grammar.follow("("),
        Set::from([String::from("("), String::from(")"), String::from("$")])
    );
    assert_eq!(
        grammar.follow(")"),
        Set::from([String::from("("), String::from(")"), String::from("$")])
    );
}

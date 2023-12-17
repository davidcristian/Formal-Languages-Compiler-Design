use super::token::TokenKind;
use automata::Automaton;

pub struct Automata {
    is_identifier: Automaton,
    is_number: Automaton,
    is_char: Automaton,
    is_string: Automaton,
}

impl Automata {
    pub fn new() -> Result<Self, String> {
        let is_identifier = match Automaton::new("../automata/input/identifier.dfa") {
            Ok(automaton) => automaton,
            Err(e) => {
                let error = format!("[identifier] {}", e);
                return Err(error);
            }
        };
        let is_number = match Automaton::new("../automata/input/number.dfa") {
            Ok(automaton) => automaton,
            Err(e) => {
                let error = format!("[number] {}", e);
                return Err(error);
            }
        };
        let is_char = match Automaton::new("../automata/input/char.dfa") {
            Ok(automaton) => automaton,
            Err(e) => {
                let error = format!("[char] {}", e);
                return Err(error);
            }
        };
        let is_string = match Automaton::new("../automata/input/string.dfa") {
            Ok(automaton) => automaton,
            Err(e) => {
                let error = format!("[string] {}", e);
                return Err(error);
            }
        };

        let automata = Self {
            is_identifier,
            is_number,
            is_char,
            is_string,
        };

        Ok(automata)
    }

    fn is_identifier(&self, value: &str) -> bool {
        self.is_identifier.validate(value)
    }

    fn is_number(&self, value: &str) -> bool {
        self.is_number.validate(value)
    }

    fn is_char(&self, value: &str) -> bool {
        self.is_char.validate(value)
    }

    fn is_string(&self, value: &str) -> bool {
        self.is_string.validate(value)
    }

    pub fn classify(&self, value: &str) -> TokenKind {
        if self.is_identifier(value) {
            TokenKind::Identifier
        } else if self.is_number(value) {
            TokenKind::Constant
        } else if self.is_char(value) {
            TokenKind::Constant
        } else if self.is_string(value) {
            TokenKind::Constant
        } else {
            TokenKind::Unknown
        }
    }
}

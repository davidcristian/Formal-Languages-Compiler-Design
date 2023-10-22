use super::scanner::Scanner;

pub trait Token {
    fn is_of(&self, current_char: char, next_char: Option<&char>) -> bool;
    fn consume(&self, scanner: &mut Scanner) -> Option<String>;
}

pub struct StringCharToken;
impl Token for StringCharToken {
    fn is_of(&self, current_char: char, _: Option<&char>) -> bool {
        current_char == '"' || current_char == '\''
    }

    fn consume(&self, scanner: &mut Scanner) -> Option<String> {
        Some(scanner.consume_string_char())
    }
}

pub struct CommentToken;
impl Token for CommentToken {
    fn is_of(&self, current_char: char, next_char: Option<&char>) -> bool {
        current_char == '-' && next_char == Some(&'-')
    }

    fn consume(&self, scanner: &mut Scanner) -> Option<String> {
        scanner.consume_comment();
        None
    }
}

pub struct ReservedToken {
    reserved_tokens: Vec<String>,
}
impl ReservedToken {
    pub fn new(reserved_tokens: &Vec<String>) -> Self {
        Self {
            reserved_tokens: reserved_tokens.clone(),
        }
    }
}
impl Token for ReservedToken {
    fn is_of(&self, current_char: char, _: Option<&char>) -> bool {
        self.reserved_tokens.contains(&current_char.to_string())
    }

    fn consume(&self, scanner: &mut Scanner) -> Option<String> {
        Some(scanner.consume_reserved_token())
    }
}

pub struct UnclassifiedToken;
impl Token for UnclassifiedToken {
    fn is_of(&self, _: char, _: Option<&char>) -> bool {
        true
    }

    fn consume(&self, scanner: &mut Scanner) -> Option<String> {
        Some(scanner.consume_general_token())
    }
}

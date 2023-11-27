use hash_map::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TOKEN_KINDS: HashMap<Token, (&'static str, usize)> = HashMap::from(
        [
            // (Token::Unknown(_), (_, 0)),

            // Identifiers and Literals
            // (Token::Identifier(_), (_, 1)),
            // (Token::Number(_), (_, 2)),
            // (Token::Char(_), (_, 2)),
            // (Token::String(_), (_, 2)),

            // Special Symbols and Operators
            (Token::Plus, ("+", 3)),
            (Token::Minus, ("-", 4)),
            (Token::Multiply, ("*", 5)),
            (Token::Divide, ("/", 6)),
            (Token::Modulo, ("%", 7)),
            (Token::Assign, ("=", 8)),
            (Token::Equal, ("==", 9)),
            (Token::NotEqual, ("!=", 10)),
            (Token::Less, ("<", 11)),
            (Token::Greater, (">", 12)),
            (Token::LessEqual, ("<=", 13)),
            (Token::GreaterEqual, (">=", 14)),
            (Token::And, ("&&", 15)),
            (Token::Or, ("||", 16)),

            // Separators
            (Token::ParenOpen, ("(", 17)),
            (Token::ParenClose, (")", 18)),
            (Token::BracketOpen, ("[", 19)),
            (Token::BracketClose, ("]", 20)),
            (Token::BraceOpen, ("{", 21)),
            (Token::BraceClose, ("}", 22)),
            (Token::Colon, (":", 23)),
            (Token::Comma, (",", 24)),

            // Reserved Words
            (Token::KeywordNumber, ("number", 25)),
            (Token::KeywordChar, ("char", 26)),
            (Token::KeywordString, ("string", 27)),
            (Token::KeywordArray, ("array", 28)),
            (Token::KeywordInput, ("input", 29)),
            (Token::KeywordOutput, ("output", 30)),
            (Token::KeywordIf, ("if", 31)),
            (Token::KeywordElse, ("else", 32)),
            (Token::KeywordWhile, ("while", 33)),

            // Special Tokens
            (Token::NewLine, ("\n", 34)),
            (Token::EOF, ("\0", 35)), // TODO: use const
        ]
    );
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Token {
    Unknown(String), // Other

    // Identifiers and Literals
    Identifier(String), // Identifiers
    Number(String),     // Numbers
    Char(String),       // Characters
    String(String),     // Strings

    // Special Symbols and Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Assign,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,

    // Separators
    ParenOpen,
    ParenClose,
    BracketOpen,
    BracketClose,
    BraceOpen,
    BraceClose,
    Colon,
    Comma,

    // Reserved Words
    KeywordNumber,
    KeywordChar,
    KeywordString,
    KeywordArray,
    KeywordInput,
    KeywordOutput,
    KeywordIf,
    KeywordElse,
    KeywordWhile,

    // Special Tokens
    NewLine, // These are implicit statement separators (like Python)
    EOF,
}

#[allow(dead_code)]
impl Token {
    pub fn key(&self) -> String {
        match self {
            // Identifiers and Literals
            Token::Identifier(value) => String::from(value),
            Token::Number(value) => String::from(value),
            Token::Char(value) => String::from(value),
            Token::String(value) => String::from(value),

            // Other
            Token::Unknown(value) => String::from(value),
            _ => match TOKEN_KINDS.get(self) {
                Some(value) => String::from(value.0),
                None => String::from("\0"), // TODO: use const
            },
        }
    }

    pub fn value(&self) -> usize {
        match self {
            // Identifiers and Literals
            Token::Identifier(_) => 1,
            Token::Number(_) => 2,
            Token::Char(_) => 2,
            Token::String(_) => 2,

            // Other
            Token::Unknown(_) => 0,
            _ => match TOKEN_KINDS.get(self) {
                Some(value) => value.1,
                None => 0,
            },
        }
    }
}

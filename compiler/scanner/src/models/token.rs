use lazy_static::lazy_static;

use super::automata::Automata;
use hash_map::HashMap;

lazy_static! {
    static ref TOKENS: HashMap<&'static str, TokenKind> = HashMap::from([
        ("+", TokenKind::Plus),
        ("-", TokenKind::Minus),
        ("*", TokenKind::Multiply),
        ("/", TokenKind::Divide),
        ("%", TokenKind::Modulo),
        ("=", TokenKind::Assign),
        ("==", TokenKind::Equal),
        ("!=", TokenKind::NotEqual),
        ("<", TokenKind::Less),
        (">", TokenKind::Greater),
        ("<=", TokenKind::LessEqual),
        (">=", TokenKind::GreaterEqual),
        ("&&", TokenKind::And),
        ("||", TokenKind::Or),
        ("(", TokenKind::ParenOpen),
        (")", TokenKind::ParenClose),
        ("[", TokenKind::BracketOpen),
        ("]", TokenKind::BracketClose),
        ("{", TokenKind::BraceOpen),
        ("}", TokenKind::BraceClose),
        (":", TokenKind::Colon),
        (";", TokenKind::SemiColon),
        (",", TokenKind::Comma),
        ("number", TokenKind::KeywordNumber),
        ("char", TokenKind::KeywordChar),
        ("string", TokenKind::KeywordString),
        ("array", TokenKind::KeywordArray),
        ("input", TokenKind::KeywordInput),
        ("output", TokenKind::KeywordOutput),
        ("if", TokenKind::KeywordIf),
        ("else", TokenKind::KeywordElse),
        ("while", TokenKind::KeywordWhile),
    ]);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TokenKind {
    Unknown, // Other

    // Identifiers and Constants
    Identifier,
    Constant,

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
    SemiColon,
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
    // NewLine, // These are implicit statement separators (like Python)
    EOF,
    //
    // Actual Literals (enum indexes are not used from here on)
    // Number,
    // Char,
    // String,
}

impl From<&str> for TokenKind {
    fn from(inner: &str) -> Self {
        if let Some(kind) = TOKENS.get(&inner) {
            return *kind;
        }

        TokenKind::Unknown
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Token {
    kind: TokenKind,
    inner: String,
    position: usize,
}

impl From<&str> for Token {
    fn from(inner: &str) -> Self {
        let kind = TokenKind::from(inner);
        Self::new(kind, inner)
    }
}

impl Token {
    pub fn new(kind: TokenKind, inner: &str) -> Self {
        Self {
            kind,
            inner: String::from(inner),
            position: 0,
        }
    }

    pub fn unknown(inner: &str) -> Self {
        Self::new(TokenKind::Unknown, inner)
    }

    pub fn get_kind(&self) -> TokenKind {
        self.kind
    }

    pub fn get_inner(&self) -> &str {
        &self.inner
    }

    pub fn key(&self) -> usize {
        // let kind = match self.kind {
        //     TokenKind::Number | TokenKind::Char | TokenKind::String => TokenKind::Constant,
        //     _ => self.kind,
        // };

        // return the index of the token kind in the enum
        self.kind as usize
    }

    pub fn value(&self) -> usize {
        self.position
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    pub fn classify(&mut self, automata: &Automata) {
        if self.kind != TokenKind::Unknown {
            return;
        }

        self.kind = if let Some(kind) = TOKENS.get(&self.inner.as_str()) {
            *kind
        } else {
            automata.classify(&self.inner)
        };
    }
}

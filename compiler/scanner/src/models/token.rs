use super::automata::Automata;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Unknown, // Other

    // Identifiers and Literals
    Identifier,
    Literal,

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

    // Actual Literals (enum indexes are not used from here on)
    Number,
    Char,
    String,
}

pub struct Token {
    kind: TokenKind,
    inner: String,
    position: usize,
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
        let kind = match self.kind {
            TokenKind::Number | TokenKind::Char | TokenKind::String => TokenKind::Literal,
            _ => self.kind,
        };

        // return the index of the token kind in the enum
        kind as usize
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

        self.kind = match self.inner.as_str() {
            "number" => TokenKind::KeywordNumber,
            "char" => TokenKind::KeywordChar,
            "string" => TokenKind::KeywordString,
            "array" => TokenKind::KeywordArray,
            "input" => TokenKind::KeywordInput,
            "output" => TokenKind::KeywordOutput,
            "if" => TokenKind::KeywordIf,
            "else" => TokenKind::KeywordElse,
            "while" => TokenKind::KeywordWhile,

            _ => {
                if automata.is_identifier(&self.inner) {
                    TokenKind::Identifier
                } else if automata.is_number(&self.inner) {
                    TokenKind::Number
                } else if automata.is_char(&self.inner) {
                    TokenKind::Char
                } else if automata.is_string(&self.inner) {
                    TokenKind::String
                } else {
                    TokenKind::Unknown
                }
            }
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers + Literals
    Ident,
    Int,

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,

    // Comparison operators
    Eq,
    NoEq,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    If,
    Else,
    True,
    False,
    Return,
}

pub const BANG: char = '!';
pub const ASSIGN: char = '=';
pub const PLUS: char = '+';
pub const MINUS: char = '-';
pub const SLASH: char = '/';
pub const ASTERISK: char = '*';
pub const LPARAN: char = '(';
pub const RPARAN: char = ')';
pub const LBRACE: char = '{';
pub const RBRACE: char = '}';
pub const COMMA: char = ',';
pub const SEMICOLON: char = ';';
pub const LT: char = '<';
pub const GT: char = '>';

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub ty: TokenType,
    pub lit: String,
}

impl Token {
    pub fn new(ty: TokenType, lit: String) -> Self {
        Self { ty, lit }
    }
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "return" => TokenType::Return,
        _ => TokenType::Ident,
    }
}

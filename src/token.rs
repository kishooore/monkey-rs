enum TokenType {
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

struct Token {
    ty: TokenType,
    lit: String
}

impl Token {
   fn new(ty: TokenType, lit: String) -> Self {
       Self { ty, lit }
   }
}
/*
const ILLEGAL: &str = "ILLEGAL";
const EOF: &str = "EOF";
const ASSIGN: &str = "=";
const PLUS: &str = "+";
const MINUS: &str = "-";
const BANG: &str = "!";
const ASTERISK: &str = "*";
const SLASH: &str = "/";
const LT: &str = "<";
const GT: &str = ">";
const EQ: &str = "==";
const NO_EQ: &str = "!=";
const COMMA: &str = ",";
const SEMICOLON: &str = ";";
const LPAREN: &str = "(";
const RPAREN: &str = ")";
const LBRACE: &str = "{";
const RBRACE: &str = "}";
const FUNCTION: &str = "FUNCTION";
const LET: &str = "LET";
const IF: &str = "IF";
const ELSE: &str = "ELSE";
const TRUE: &str = "TRUE";
const FALSE: &str = "FALSE";
const RETURN: &str = "RETURN";
*/


fn lookup_ident(ident: &str) -> TokenType {
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


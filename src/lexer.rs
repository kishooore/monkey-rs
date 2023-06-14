use crate::token::*;

struct Lexer {
    input:         String,
    position:      usize,
    read_position: usize,
    ch:            Option<char>,
}

impl Lexer {
    fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: None
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_white_space();

        let token = match self.ch.unwrap() {
            ASSIGN => Self::new_token(TokenType::Assign, self.ch.unwrap()),
            PLUS => Self::new_token(TokenType::Plus, self.ch.unwrap()),
            LPARAN => Self::new_token(TokenType::LParen, self.ch.unwrap()),
            RPARAN => Self::new_token(TokenType::RParen, self.ch.unwrap()),
            LBRACE => Self::new_token(TokenType::LBrace, self.ch.unwrap()),
            RBRACE => Self::new_token(TokenType::RBrace, self.ch.unwrap()),
            COMMA => Self::new_token(TokenType::Comma, self.ch.unwrap()),
            SEMICOLON => Self::new_token(TokenType::Semicolon, self.ch.unwrap()),
            _ => {
                if Self::is_letter(self.ch.unwrap()) {
                    let literal = self.read_identifier();
                    let token_type = lookup_ident(literal);
                    return Token::new(token_type, literal.to_string());
                } else if Self::is_digit(self.ch.unwrap()) {
                    return Token::new(TokenType::Int, self.read_number().to_string());
                } else {
                    return Self::new_token(TokenType::Illegal, self.ch.unwrap());
                }
            }
        };
        self.read_char();

        token
    }

    fn skip_white_space(&mut self) {
        while self.ch.unwrap() == ' ' || self.ch.unwrap() == '\t' || self.ch.unwrap() == '\n' || self.ch.unwrap() == '\r' {
            self.read_char();
        }
    }

    fn new_token(ty: TokenType, ch: char) -> Token {
        Token::new(ty, ch.to_string())
    }

    fn is_digit(ch: char) -> bool {
        ch >= '0' && ch <= '9'
    }

    fn is_letter(ch: char) -> bool {
        (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch == '_'
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while Self::is_letter(self.ch.unwrap()) {
            self.read_char();
        }
        &self.input[position..self.position]
    }

    fn read_number(&mut self) -> &str {
        let position = self.position;
        while Self::is_digit(self.ch.unwrap()) {
            self.read_char();
        }
        &self.input[position..self.position]
    }
}

#[cfg(test)]
mod tests {
    use crate::token::{Token, TokenType};

    use super::*;

    #[test]
    fn next_token_with_simple_tokens() -> Result<(), String> {
        let input = "=+(){},;";

        let mut lexer = Lexer::new(input.to_string());
        let tests = vec!(
            Token::new(TokenType::Eq, "=".to_string()),
            Token::new(TokenType::Plus, "+".to_string()),
            Token::new(TokenType::LParen, "(".to_string()),
            Token::new(TokenType::RParen, ")".to_string()),
            Token::new(TokenType::LBrace, "{".to_string()),
            Token::new(TokenType::RBrace, "}".to_string()),
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
        );
        for (i,test) in tests.iter().enumerate() {
            let tok = lexer.next_token();
            if tok.lit != test.lit {
                return Err(String::from(format!("test {:?} failed. literal wrong. expected={:?}, got={:?}", i, test.lit, tok.lit)));
            }
            if tok.ty != test.ty {
                return Err(String::from(format!("test {:?} failed. tokentype wrong. expected={:?}, got={:?}", i, test.lit, tok.lit)));
            }
        }
        Ok(())
    }

    #[test]
    fn next_token_with_keywords_and_identifier() -> Result<(), String> {
        let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
	x + y;
};

let result = add(five, ten);";

        let mut lexer = Lexer::new(input.to_string());
        let tests = vec!(
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Ident, "five".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Int, "5".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Ident, "ten".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Int, "10".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Ident, "add".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Function, "fn".to_string()),
            Token::new(TokenType::LParen, "(".to_string()),
            Token::new(TokenType::Ident, "x".to_string()),
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::Ident, "y".to_string()),
            Token::new(TokenType::RParen, ")".to_string()),
            Token::new(TokenType::LBrace, "{".to_string()),
            Token::new(TokenType::Ident, "x".to_string()),
            Token::new(TokenType::Plus, "+".to_string()),
            Token::new(TokenType::Ident, "y".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::RBrace, "}".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Ident, "result".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Ident, "add".to_string()),
            Token::new(TokenType::LParen, "(".to_string()),
            Token::new(TokenType::Ident, "five".to_string()),
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::Ident, "ten".to_string()),
            Token::new(TokenType::RParen, ")".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
        );

        for (i,test) in tests.iter().enumerate() {
            let tok = lexer.next_token();
            if tok.lit != test.lit {
                return Err(String::from(format!("test {:?} failed. literal wrong. expected={:?}, got={:?}", i, test.lit, tok.lit)));
            }
            if tok.ty != test.ty {
                return Err(String::from(format!("test {:?} failed. tokentype wrong. expected={:?}, got={:?}", i, test.ty, tok.ty)));
            }
        }
        Ok(())
    }
}

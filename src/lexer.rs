use crate::token::*;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: None,
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

    fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            self.input.chars().nth(self.read_position)
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_white_space();

        if let Some(ch) = self.ch {
            let token = match ch {
                '<' => Self::new_token(TokenType::Lt, self.ch.unwrap()),
                '>' => Self::new_token(TokenType::Gt, self.ch.unwrap()),
                '=' => {
                    if Some('=') == self.peek_char() {
                        self.read_char();
                        Token::new(TokenType::Eq, "==".to_string())
                    } else {
                        Token::new(TokenType::Assign, "=".to_string())
                    }
                }
                '!' => {
                    if Some('=') == self.peek_char() {
                        self.read_char();
                        Token::new(TokenType::NoEq, "!=".to_string())
                    } else {
                        Token::new(TokenType::Bang, "!".to_string())
                    }
                },
                '+' => Self::new_token(TokenType::Plus, self.ch.unwrap()),
                '-' => Self::new_token(TokenType::Minus, self.ch.unwrap()),
                '/' => Self::new_token(TokenType::Slash, self.ch.unwrap()),
                '(' => Self::new_token(TokenType::LParen, self.ch.unwrap()),
                ')' => Self::new_token(TokenType::RParen, self.ch.unwrap()),
                '{' => Self::new_token(TokenType::LBrace, self.ch.unwrap()),
                '}' => Self::new_token(TokenType::RBrace, self.ch.unwrap()),
                ',' => Self::new_token(TokenType::Comma, self.ch.unwrap()),
                ';' => Self::new_token(TokenType::Semicolon, self.ch.unwrap()),
                '*' => Self::new_token(TokenType::Asterisk, self.ch.unwrap()),
                _ => {
                    if Self::is_letter(self.ch.unwrap()) {
                        let literal = self.read_identifier();
                        let token_type = lookup_ident(literal);
                        return Some(Token::new(token_type, literal.to_string()))
                    } else if Self::is_digit(self.ch.unwrap()) {
                        return Some(Token::new(TokenType::Int, self.read_number().to_string()))
                    } else {
                        return Some(Self::new_token(TokenType::Illegal, self.ch.unwrap()))
                    }
                }
            };
            self.read_char();
            Some(token)
        } else {
            None
        }

    }

    fn skip_white_space(&mut self) {
        while self.ch != None && (self.ch.unwrap() == ' '
            || self.ch.unwrap() == '\t'
            || self.ch.unwrap() == '\n'
            || self.ch.unwrap() == '\r')
        {
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
        let tests = vec![
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Plus, "+".to_string()),
            Token::new(TokenType::LParen, "(".to_string()),
            Token::new(TokenType::RParen, ")".to_string()),
            Token::new(TokenType::LBrace, "{".to_string()),
            Token::new(TokenType::RBrace, "}".to_string()),
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
        ];
        for (i, test) in tests.iter().enumerate() {
            let tok = lexer.next_token().unwrap();
            if tok.lit != test.lit {
                return Err(String::from(format!(
                    "test {:?} failed. literal wrong. expected={:?}, got={:?}",
                    i, test.lit, tok.lit
                )));
            }
            if tok.ty != test.ty {
                return Err(String::from(format!(
                    "test {:?} failed. tokentype wrong. expected={:?}, got={:?}",
                    i, test.ty, tok.ty
                )));
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
        let tests = vec![
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
        ];

        for (i, test) in tests.iter().enumerate() {
            let tok = lexer.next_token().unwrap();
            if tok.lit != test.lit {
                return Err(String::from(format!(
                    "test {:?} failed. literal wrong. expected={:?}, got={:?}",
                    i, test.lit, tok.lit
                )));
            }
            if tok.ty != test.ty {
                return Err(String::from(format!(
                    "test {:?} failed. tokentype wrong. expected={:?}, got={:?}",
                    i, test.ty, tok.ty
                )));
            }
        }
        Ok(())
    }

    #[test]
    fn next_token_with_math_operators() -> Result<(), String> {
        let input = "!-/*5;
5 < 10 > 5;
10 == 10;
9 != 10;";

        let mut lexer = Lexer::new(input.to_string());
        let tests = vec![
            Token::new(TokenType::Bang, "!".to_string()),
            Token::new(TokenType::Minus, "-".to_string()),
            Token::new(TokenType::Slash, "/".to_string()),
            Token::new(TokenType::Asterisk, "*".to_string()),
            Token::new(TokenType::Int, "5".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Int, "5".to_string()),
            Token::new(TokenType::Lt, "<".to_string()),
            Token::new(TokenType::Int, "10".to_string()),
            Token::new(TokenType::Gt, ">".to_string()),
            Token::new(TokenType::Int, "5".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Int, "10".to_string()),
            Token::new(TokenType::Eq, "==".to_string()),
            Token::new(TokenType::Int, "10".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Int, "9".to_string()),
            Token::new(TokenType::NoEq, "!=".to_string()),
            Token::new(TokenType::Int, "10".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
        ];
        for (i, test) in tests.iter().enumerate() {
            let tok = lexer.next_token().unwrap();
            if tok.lit != test.lit {
                return Err(String::from(format!(
                    "test {:?} failed. literal wrong. expected={:?}, got={:?}",
                    i, test.lit, tok.lit
                )));
            }
            if tok.ty != test.ty {
                return Err(String::from(format!(
                    "test {:?} failed. tokentype wrong. expected={:?}, got={:?}",
                    i, test.ty, tok.ty
                )));
            }
        }
        Ok(())
    }

    #[test]
    fn next_token_with_conditional_statements() -> Result<(), String> {
        let input = "if (5 < 10) {
	return true;
} else {
	return false;
}";
    
        let mut lexer = Lexer::new(input.to_string());
        let tests = vec![
            Token::new(TokenType::If, "if".to_string()),
            Token::new(TokenType::LParen, "(".to_string()),
            Token::new(TokenType::Int, "5".to_string()),
            Token::new(TokenType::Lt, "<".to_string()),
            Token::new(TokenType::Int, "10".to_string()),
            Token::new(TokenType::RParen, ")".to_string()),
            Token::new(TokenType::LBrace, "{".to_string()),
            Token::new(TokenType::Return, "return".to_string()),
            Token::new(TokenType::True, "true".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::RBrace, "}".to_string()),
            Token::new(TokenType::Else, "else".to_string()),
            Token::new(TokenType::LBrace, "{".to_string()),
            Token::new(TokenType::Return, "return".to_string()),
            Token::new(TokenType::False, "false".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::RBrace, "}".to_string()),
        ];
        for (i, test) in tests.iter().enumerate() {
            let tok = lexer.next_token().unwrap();
            if tok.lit != test.lit {
                return Err(String::from(format!(
                    "test {:?} failed. literal wrong. expected={:?}, got={:?}",
                    i, test.lit, tok.lit
                )));
            }
            if tok.ty != test.ty {
                return Err(String::from(format!(
                    "test {:?} failed. tokentype wrong. expected={:?}, got={:?}",
                    i, test.ty, tok.ty
                )));
            }
        }
        Ok(())
    }
}

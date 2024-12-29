use crate::tokens::{Token, TokenType};

pub struct TokenStream<'a> {
    tokens: std::slice::Iter<'a, Token>,
    current: Option<&'a Token>,
}

impl<'a> TokenStream<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        let mut stream = Self {
            tokens: tokens.iter(),
            current: None,
        };
        stream.advance(); // Load the first token
        stream
    }

    pub fn advance(&mut self) -> Option<&'a Token> {
        self.current = self.tokens.next();
        self.current
    }

    pub fn advance_prev(&mut self) -> Option<&'a Token> {
        let tmp = self.current;
        self.current = self.tokens.next();
        tmp
    }

    pub fn peek(&self) -> Option<&Token> {
        self.current
    }

    pub fn expect(&mut self, expected: &TokenType) -> Result<(), String> {
        if let Some(token) = self.current {
            if &token.token == expected {
                self.advance(); // Consume the token
                Ok(())
            } else {
                Err(format!("Expected {:?}, found {:?}", expected, token))
            }
        } else {
            Err("Unexpected end of tokens".to_string())
        }
    }
}
